pub fn find_numbers(line: &str) -> Option<(u32, u32)> {
    let numeric_opt = FindResult::find_numeric(line);
    let literal_opt = FindResult::find_literal(line);
    
    #[cfg(test)]
    {
        println!("Numeric result: {:?}", numeric_opt);
        println!("Literal result: {:?}", literal_opt);
    }

    let find_result : FindResult = if numeric_opt.is_none() {
        literal_opt?
    } else if literal_opt.is_none() {
        numeric_opt.unwrap()
    } else {
        let mut n = numeric_opt.unwrap();
        let l = literal_opt.unwrap();
        n.combine(l);
        n
    };

    Some(find_result.get_values(line))
}

#[derive(Clone)]
#[derive(Debug)]
struct FindRange {
    index: usize,
    length: usize
}

#[derive(Clone)]
enum FindDirection {
    Forward,
    Backward
}

// The basic enum is a value type
impl Copy for FindDirection {}

#[derive(Debug)]
struct FindResult {
    forward: FindRange,
    backward: FindRange,
}

const NUM_LITERALS: [&str;9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn literal_parse(line: &str) -> Option<u32> {
    match line {
        // I checked, there is no "zero"
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None
    }
}

impl FindRange {
    fn find_numeric(line: &str, direction: FindDirection) -> Option<Self> {
        let index = match direction {
            FindDirection::Forward => line.find(char::is_numeric),
            FindDirection::Backward => line.rfind(char::is_numeric)
        }?;
        Some(Self {
            index,
            length: 1,
        })
    }

    fn find_literal(line: &str, direction: FindDirection) -> Option<Self> {
        let mut result: Option<Self> = None;
        for lit in NUM_LITERALS {
            let new_opt = Self::find_single_literal(line, lit, direction);
            if let Some(r) = &mut result {
                if let Some(new_result) = new_opt {
                    r.combine(new_result, direction);
                }
            } else {
                result = new_opt;
            }
        }

        result
    }

    fn find_single_literal(line: &str, literal: &str, direction: FindDirection) -> Option<Self> {
        let index = match direction {
            FindDirection::Forward => line.find(literal),
            FindDirection::Backward => line.rfind(literal),
        }?;

        Some(Self {
            index,
            length: literal.len(),
        })
    }

    fn slice<'a>(&self, line: &'a str) -> &'a str {
        let end_index = self.index + self.length;
        &line[self.index..end_index]
    }

    fn slice_after<'a>(&self, line: &'a str) -> &'a str {
        let index = self.index + self.length;
        &line[index..]
    }

    fn get_value(&self, line: &str) -> u32 {
        let substr = self.slice(line);
        if self.length == 1 {
            substr.parse::<u32>().expect("Found invalid number.")
        } else {
            literal_parse(substr).expect("Found invalid number.")
        }
    }

    fn expand(&mut self, other: &Self) {
        self.index += other.index + other.length;
    }

    fn combine(&mut self, other: Self, direction: FindDirection) {
        if self.index == other.index {
            // Something really bad has happened if we found different numbers at the same index
            if other.length != self.length {
                println!("Self {:?} and other {:?}", self, other);
            }
            assert_eq!(self.length, other.length);
            return;
        }

        let less_than = self.index < other.index;
        let better = match direction { 
            FindDirection::Forward => less_than,
            FindDirection::Backward => !less_than
        };

        if !better {
            *self = other;
        }
    }
}

impl FindResult {
    fn find_numeric(line: &str) -> Option<Self> {
        Self::find_impl(line, FindRange::find_numeric)
    }

    fn find_literal(line: &str) -> Option<Self> {
        Self::find_impl(line, FindRange::find_literal)
    }

    fn find_impl<F>(line: &str, finder: F) -> Option<Self>
        where F: Fn(&str, FindDirection) -> Option<FindRange>
    {
        let forward = finder(line, FindDirection::Forward)?;

        // Search backwards on the remaining string
        let remaining = forward.slice_after(line);
        let backward_opt = finder(remaining, FindDirection::Backward);

        let backward = match backward_opt {
            Some(mut val) => { val.expand(&forward); val } // Realign the index to the value for the overall string
            None => forward.clone() // If we found nothing, then the value is just the same as the forward search
        };

        Some(Self {
            forward,
            backward,
        })
    }

    fn get_values(&self, line: &str) -> (u32, u32) {
        (self.forward.get_value(line),
        self.backward.get_value(line))
    }

    fn combine(&mut self, other: Self) {
        self.forward.combine(other.forward, FindDirection::Forward);
        self.backward.combine(other.backward, FindDirection::Backward);
    }

}


