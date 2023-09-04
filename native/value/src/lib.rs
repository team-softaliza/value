use rustler::{Atom, Env, Error, ListIterator, Term, Encoder};
mod atoms {
    rustler::atoms! {
      nil
    }
}
rustler::init!("Elixir.Value", [nif_path, get, get_many]);

#[rustler::nif(name = "nif_path")]
fn nif_path<'a>( env: Env<'a>, fields0: String) -> Result<Term<'a>, Error> {
    let replaced = fields0.replace(" ", "");
    let mut fields: Vec<&str> = replaced.split(",").collect();
    fields.sort_by(|a,b|a.cmp(b));
    fields.dedup();

    let mut base: &str = "";
    let mut list: Vec<(&str, Vec<Vec<&str>>)> = Vec::new();
    let mut deep_tmp:Vec<Vec<&str>> = Vec::new();

    for (i, p) in fields.iter().enumerate() {
      let s: Vec<&str> = p.split(".").collect();
      if s.len() > 1 && (base == "" || base == s[0]){
        base = s[0];
        if i + 1 < fields.len() {
          deep_tmp.push(s);
        } else if i + 1 == fields.len() {
          deep_tmp.push(s);
          let tmp0:Vec<Vec<&str>> = deep_tmp.clone().into_iter().fold([].to_vec(),|mut acc, mut a|{
            a.remove(0);
            acc.push(a);
            acc
          });
          list.push((base, tmp0));
          base = "";
          deep_tmp.clear();
        }
      } else {
        if deep_tmp.len() > 0 {
          let tmp0:Vec<Vec<&str>> = deep_tmp.clone().into_iter().fold([].to_vec(),|mut acc, mut a|{
            a.remove(0);
            acc.push(a);
            acc
          });
          list.push((base, tmp0));
          base = "";
          deep_tmp.clear();
          list.push((s[0], [].to_vec()))
        } else {

          list.push((s[0], [].to_vec()))
        }
      }
    }
    let enc = list.encode(env);
    Ok(enc)
}


#[rustler::nif(name = "nif_get")]
fn get<'a>(
    env: Env<'a>,
    term: Term<'a>,
    field: String,
    optional: Term<'a>,
) -> Result<Term<'a>, Error> {
    get_in_depth(env, field.split(".").collect(), term, optional)
}
fn get_in_depth<'a>(
    env: Env<'a>,
    path: Vec<&str>,
    term: Term<'a>,
    optional: Term<'a>,
) -> Result<Term<'a>, Error> {
    let mut result = term;
    let mut p0 = path.clone();
    if path.len() > 0  {
      if term.is_map() {
        let field = p0.remove(0);
        match result.map_get(field) {

          Ok(result0) => result = get_in_depth(env, p0.clone(), result0, optional)?,
          _ => match Atom::from_str(env, field) {
              Ok(fieldstr) => match result.map_get(fieldstr) {
                  Ok(result0) => result = get_in_depth(env, p0.clone(), result0, optional)?,
                  _ => result = optional,
              },
              _ => result = optional,
          },
        };
      }
      if term.is_list() {
        let mut t0:Vec<Term> = Vec::new();
        for t in term.into_list_iterator()? {
          let r = get_in_depth(env, p0.clone(), t, optional)?;
          t0.push(r)
        }
        result = t0.encode(env);
      }
    }
    Ok(result)
}

#[rustler::nif(name = "nif_get_many")]
fn get_many<'a>(env: Env<'a>, term: Term<'a>, fields: Term) -> Result<Term<'a>, Error> {
    get_in_depth_many(
        env,
        fields.into_list_iterator()?,
        term,
    )
}
fn get_in_depth_many<'a>(
    env: Env<'a>,
    paths: ListIterator,
    term: Term<'a>,
) -> Result<Term<'a>, Error> {
    let map = rustler::types::map::map_new(env);
    let mut final_result = rustler::types::map::map_new(env);
    let mut acc = rustler::types::map::map_new(env);
    for path in paths {
        let mut arr: Vec<(Term, Term)> = Vec::new();
        match rustler::types::tuple::get_tuple(path).as_deref() {
          Ok(&[field, deep]) => {
            // println!("{:?} e deep {:?}", field, deep);
            let field0:&str = field.decode()?;
            let length = match deep.list_length() {
              Ok(length) => length,
              _ => 0
            };
            if length > 0 {
              // let some0 = get_in_depth(env, t1, term, atoms::nil().to_term(env))?;
              // println!("deep {:?}, acc: {:?}, some0: {:?}", deep, acc, some0);
              for t0 in deep.into_list_iterator()? {
                let mut t1:Vec<&str> = Vec::new();
                t1.push(field0);
                for x in t0.into_list_iterator()? {
                  let d:&str = x.decode()?;
                  let map0 = match acc.map_get(x) {
                    Ok(result) => result,
                    _ => map
                  };
                  // println!("field {:?} e acc {:?} e map0 {:?}", field, acc, map0);
                  arr.push((x, map0));
                  t1.push(d);
                }
                let field1 = t1[1];
                // println!("deep {:?}", t1);
                let some = get_in_depth(env, t1, term, atoms::nil().to_term(env))?;
                let rev: Vec<(Term, Term)> = arr.clone().into_iter().rev().collect::<Vec<(Term, Term)>>();

                 let final_result0 = rev.iter().fold(some, | acc, (field, val)| {
                  match val.map_put(field, acc) {
                    Ok(x1) => x1,
                    Err(_x1) => atoms::nil().to_term(env)
                  }
                });

                let getted0 = match final_result0.map_get(field1) {
                  Ok(result) => result,
                  _ => final_result0
                 };
                arr.clear();
                acc = acc.map_put(field1, getted0)?
              }

              final_result = final_result.map_put(field, acc)?
            } else {

              let some = get_in_depth(env, [field0].to_vec(), term, atoms::nil().to_term(env))?;
              final_result = final_result.map_put(field, some)?
            }
          },
          _ => {}
        }
    }
    Ok(final_result)
}