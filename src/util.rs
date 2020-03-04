use ndarray::Array2;
use serde::{ser::SerializeSeq, Serialize, Serializer};
use warp::{http::StatusCode, reply::Response, Reply};

/// Wraps an Array2 so that it serializes as a simple array of arrays.
#[derive(Debug)]
pub struct Array2SerializeWrapper<T>(pub Array2<T>);

#[derive(Debug, Serialize)]
struct ErrorMessage<'a> {
    code: u16,
    message: &'a str,
}

pub fn error_response(code: StatusCode, message: &'_ str) -> Response {
    let json = warp::reply::json(&ErrorMessage { code: code.as_u16(), message });

    warp::reply::with_status(json, code).into_response()
}

impl<T> Serialize for Array2SerializeWrapper<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.nrows()))?;
        for row in self.0.genrows() {
            let row = row.as_slice().unwrap();
            seq.serialize_element(row)?;
        }

        seq.end()
    }
}
