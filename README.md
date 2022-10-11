# hyper::client mock 

A mock of hyper::client to test tower::Services like axum::router. 

## Usage
See the tests of the lib. Create an instance of the Service you want to test and create a new client with `MockClient::new(app)`. The MockClient has an interface similar to hyper::client with `MockClient::request(hyper::Request)` being your main method but returning a simple `Future<Response<Body>>`. 

## License

This project is licensed under the MIT license.

