`ism`
=====
Intermediate structured message.



Goal
----
- A simpler form for intermediate structured message to connect dynamic world to static world.
- JSON is quite good option, but we want something better.
    ```
    Dynamic World -> ISM Message (JSON subset) -> Static World
    ```



Design Choices
--------------
- `Message` is a strict subset of JSON.
- It supports only JSON String/List/Object.
    - No explicit `null`. Missing value is represented as absence of field.
    - No number. No boolean. All strings.
- Having an intermediate type cannot be more efficient than direct encoding/decoding.
    - So we do not aim the best performance.
    - This is for easier design & development.
    - Still would be good enough as Rust's `String` has small string optimization.
- Untagged to support better fitting with native JSON format.
- Cloneable for convenience, but not recommended. (`O(n)`)
    - Consuming out the message is recommended when you converting it to something else.
- Rust schema:
    ```rust
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Message {
        Value(Value),
        List(List),
        Table(Table),
    }
    pub type Value = CompactString;
    pub type List = Vec<Message>;
    pub type Table = HashMap<Value, Message>;
    ```
- JSON encoded instance examples:
    ```json
    "hello"

    ["a", "b", "c"]

    {"x": "y"}
    
    [
        "a",
        ["b", "c"],
        {"d": "e"}
    ]

    {
        "msg": "hello",
        "list": ["a", "b"],
        "data": {
            "x": "y"
        }
    }
    ```
- Provides declarative way to encode/decode into/from concrete types.
    - The source/target concrete types are also restricted.
    ```rust
    #[derive(ism::Serialize, ism::Deserialize)]
    struct MyMessage1 {
        msg2: String,
        list3: Vec<u32>,
        data4: HashMap<String, Vec<f64>>,
    }
    ```
    - Source/target Concrete types does not have same restrictions.
        - They can have more leaf node value types.
            - Any type is supported if it supports `ToString`/`FromStr`.
        - For container types:
            - `Vec<T>` where `T` is supported.
            - `HashMap<String, T>` where `T` is supported.
            - `Option<T>` where `T` is supported.



Crates/Modules
--------------
- `ism` crate:
    - Core implementation.
    - `crate::text` module:
        - Text-based implementation.
        - `pub enum Message`
            - `pub fn serialize`
            - `pub fn deserialize`
        - `pub struct Value`
            - Efficient & easy-to-use immutable text type.
            - Something like `Substring` in Swift.
                - Shares immutable source string buffer. (`Rc<Vec<u8>>`)
                - Substring is shared buffer with range.
                - Substring copy/clone is O(1).
                - 
    - `crate::binary`
        - Binary-based implementation.
        - (TBD)
- `ism_derive` crate: 
    - Procedural macros for deriving `Serialize` and `Deserialize` traits.
    - `crate::Serialize` macro.
    - `crate::Deserialize` macro.



Tests
-----
- All tests are divided into happy, edge, evil cases.
- `ism_tests` crate:
    - `crate::text` module:
        - Contains serialization from/into JSON.
        - Contains test cases for `ism::text` module.
    - `crate::binary` module:
        - (TBD)
    - `crate::derive` module:
        - Contains test cases for procedural macros.
        - Test code-to-code derivation test.
        - Test actual compilability for the derivations.
        - Use special crates (e.g. `trybuild`) for all compilability test cases.
            - And collect the result within testing framework.
            - Even happy cases can fail if there's a bug.
            