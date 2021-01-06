use crate::xgl::parse;

#[test]
fn test_parse_mysql57_log() {
  let reader = b"/usr/local/opt/mysql@5.7/bin/mysqld, Version: 5.7.32-log (Homebrew). started with:
Tcp port: 0  Unix socket: (null)
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

#[test]
fn test_parse_mysql56_log() {
  let reader = b"/usr/local/opt/mysql@5.6/bin/mysqld, Version: 5.6.50-log (Homebrew). started with:
Tcp port: 0  Unix socket: (null)
Time                 Id Command    Argument
210106 21:05:35	    1 Connect	root@localhost on
\t\t    1 Query\tselect @@version_comment limit 1
\t\t    1 Query\tselect USER()
210106 21:05:50\t    1 Query\tselect 1
210106 21:05:57\t    1 Query\tselect 1
from dual
210106 21:05:59\t    1 Quit\t
" as &[u8];
  let mut jsonl = vec![];

  parse(reader, |header, arg| {
    jsonl.push(format!("{:?} {}", header, arg));
  })
  .unwrap();

  assert_eq!(
    jsonl,
    vec![
      r#"Header { time: "210106 21:05:35", id: "1", command: "Connect" } root@localhost on"#,
      r#"Header { time: "210106 21:05:35", id: "1", command: "Query" } select @@version_comment limit 1"#,
      r#"Header { time: "210106 21:05:35", id: "1", command: "Query" } select USER()"#,
      r#"Header { time: "210106 21:05:50", id: "1", command: "Query" } select 1"#,
      concat!(
        r#"Header { time: "210106 21:05:57", id: "1", command: "Query" } "#,
        "select 1\nfrom dual"
      ),
      r#"Header { time: "210106 21:05:59", id: "1", command: "Quit" } "#,
    ]
  );
}
