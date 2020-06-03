extern crate url;
use url::form_urlencoded;

pub fn build_encoded_url(base_url String) {
    let encoded: String = form_urlencoded::Serializer::new(
         String::from(base_url))
        .append_pair("foo", "bar & baz")
        .append_pair("saison", "Été+hiver")
        .finish();

    println!("{:?}", encoded);
}
