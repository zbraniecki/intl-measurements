use std::fmt;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrintOptions {
    _type: PrintType,
    style: PrintStyle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrintType {
    Text,
    JSON
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrintStyle {
    Compact,
    Detailed
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Test {
    pub id: String,
    pub description: String,
    pub unit: TestUnit,
}

impl Test {
    fn print<W: fmt::Write>(&self, mut f: W, result: &TestResult, options: PrintOptions) -> fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        if options._type == PrintType::Text {
            if options.style == PrintStyle::Compact {
                write!(f, "{}: {}", self.id, result)?;
            } else {
                write!(f, "================")?;
                write!(f, "Test: {}", self.id)?;
                write!(f, "Description: {}", self.description)?;
                write!(f, "Value: {}", result)?;
            }
        } else {
            let result = if options.style == PrintStyle::Compact {
                #[derive(Serialize, Deserialize)]
                struct JSONTestResult {
                    id: String,
                    result: u128,
                    unit: String,
                };
                let result = JSONTestResult {
                    id: self.id.clone(),
                    unit: self.unit.to_string(),
                    result: result.value
                };
                serde_json::to_string(&result).unwrap()
            } else {
                #[derive(Serialize, Deserialize)]
                struct JSONTestResult {
                    id: String,
                    description: String,
                    result: u128,
                    unit: String,
                };
                let result = JSONTestResult {
                    id: self.id.clone(),
                    description: self.description.clone(),
                    unit: self.unit.to_string(),
                    result: result.value
                };
                serde_json::to_string_pretty(&result).unwrap()
            };
            f.write_str(&result)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TestUnit {
    Time(TestUnitTime),
    Size(TestUnitSize),
}

impl From<&str> for TestUnit {
    fn from(input: &str) -> Self {
        input.parse().expect("Failed to parse unit")
    }
}

impl fmt::Display for TestUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Time(t) => t.fmt(f),
            Self::Size(t) => t.fmt(f),
        }
    }
}

impl FromStr for TestUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TestUnitTime::from_str(s).map(Into::into)
            .or_else(|_| TestUnitSize::from_str(s).map(Into::into))
    }
}

impl From<TestUnitTime> for TestUnit {
    fn from(input: TestUnitTime) -> Self {
        Self::Time(input)
    }
}

impl From<TestUnitSize> for TestUnit {
    fn from(input: TestUnitSize) -> Self {
        Self::Size(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TestUnitTime {
    Nano,
    Micro,
    Mili,
    Seconds
}

impl fmt::Display for TestUnitTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nano => f.write_str("ns"),
            Self::Micro => f.write_str("us"),
            Self::Mili => f.write_str("ms"),
            Self::Seconds => f.write_str("s"),
        }
    }
}

impl FromStr for TestUnitTime {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ns" => Ok(Self::Nano),
            "us" => Ok(Self::Micro),
            "ms" => Ok(Self::Mili),
            "s" => Ok(Self::Seconds),
            _ => Err(())
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TestUnitSize {
    Bytes,
    Kilobytes,
}

impl fmt::Display for TestUnitSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bytes => f.write_str("b"),
            Self::Kilobytes => f.write_str("kb"),
        }
    }
}

impl FromStr for TestUnitSize {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" => Ok(Self::Bytes),
            "kb" => Ok(Self::Kilobytes),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TestResult {
    pub value: u128,
    pub unit: TestUnit,
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestSuitResult {
    pub tests: Vec<(Test, TestResult)>
}

impl TestSuitResult {
    pub fn print(&self) -> fmt::Result {
        let options = PrintOptions {
            _type: PrintType::JSON,
            style: PrintStyle::Compact,
        };

        let mut s = String::new();

        if options._type == PrintType::JSON {
            writeln!(s, "[")?;
            for (i, (test, result)) in self.tests.iter().enumerate() {
                test.print(&mut s, result, options)?;
                if i + 1 == self.tests.len() {
                    writeln!(s, "")?;
                } else {
                    writeln!(s, ",")?;
                }
            }
            write!(s, "]")?;
        } else {
            for (test, result) in &self.tests {
                test.print(&mut s, result, options)?;
            }
        }
        println!("{}", s);
        Ok(())
    }
}
