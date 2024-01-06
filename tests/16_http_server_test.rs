use std::{
    fs::{set_permissions, Permissions},
    os::unix::fs::PermissionsExt,
};

use assert_cmd::{assert::OutputAssertExt, Command};

// #[test]
// fn test_success() {
//     let mut server_cmd = Command::cargo_bin("16_http_server").unwrap();

//     let mut assert = Command::new("nc")
//         .arg("localhost")
//         .arg("8087")
//         .write_stdin("GET /index.html HTTP/1.1\nHost: localhost\n\n")
//         .unwrap()
//         .assert();

//     assert.success().stdout(
//         "HTTP/1.1 200 OK\r\n
// Content-length: 55\r\n
// Content-Type: text/html\r\n
// Connection: close\r\n
// \r\n
// <!DOCTYPE html>\r\n
// <html>\r\n
//   <h1>hello, guys.</h1>\r\n
// </html>\r\n",
//     );
// }
