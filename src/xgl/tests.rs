use crate::xgl::parse;

#[test]
fn test_parse_mysql57_log() {
  let reader = b"Tcp port: 0  Unix socket: (null)
Time                 Id Command    Argument
2021-01-06T05:34:08.229302Z\t    2 Connect\troot@localhost on  using Socket
2021-01-06T05:34:08.230690Z\t    2 Query\tselect @@version_comment limit 1
2021-01-06T05:34:08.248396Z\t    2 Query\tselect USER()
2021-01-06T05:35:40.932704Z\t    3 Query\tselect * from
store
limit 1
2021-01-06T05:35:41.995052Z\t    3 Quit\t
" as &[u8];
  let mut jsonl = vec![];

  parse(reader, |header, arg| {
    jsonl.push(format!("{:?} {}", header, arg));
  })
  .unwrap();

  assert_eq!(
    jsonl,
    vec![
      r#"Header { time: "2021-01-06T05:34:08.229302Z", id: "2", command: "Connect" } root@localhost on  using Socket"#,
      r#"Header { time: "2021-01-06T05:34:08.230690Z", id: "2", command: "Query" } select @@version_comment limit 1"#,
      r#"Header { time: "2021-01-06T05:34:08.248396Z", id: "2", command: "Query" } select USER()"#,
      concat!(
        r#"Header { time: "2021-01-06T05:35:40.932704Z", id: "3", command: "Query" } "#,
        "select * from\nstore\nlimit 1"
      ),
      r#"Header { time: "2021-01-06T05:35:41.995052Z", id: "3", command: "Quit" } "#,
    ]
  );
}
