/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/


#[derive(Debug)]
struct Stack<T> {
	
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
	fn len(&self) -> usize {
		self.data.len()
	}
	fn clear(&mut self) {
		
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		
	}
	fn pop(&mut self) -> Option<T> {
		// TODO
		self.data.pop()
	}
	fn peek(&self) -> Option<&T> {
		
		self.data.last()
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		
		self.data.last_mut()
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool
{
	//TODO
	let mut stack = Stack::new();
	for c in bracket.chars(){
		match c {
			'(' | '{' | '[' => { stack.push(c) },
		')' => {
			if stack.pop() != Some('('){
				return false;
			}
		},
		']' =>{
			if stack.pop() != Some('['){
				return false;
			}
		},
		'}' => {
			if stack.pop() != Some('{') {
				return false;
			}
		},
		_  =>  {}
		}
		
	}
	stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}