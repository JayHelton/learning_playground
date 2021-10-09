async function delegate() {
	console.log('third');// immediately executed
}

async function main() {
	console.log('second');// immediately executed
	await delegate(); // after delete gate is executed, this function yields back control of the main thread, the remaining instructions are added to the event loops
	console.log('fifth');// added to event loop first, since its after an await (basically calling .then) 
}

console.log('first'); // immediately executed
main().then(() => {
	console.log("Sixth"); // added to the event loop last since its in the final .then
})

console.log('fourth');

let thing = new Promise((resolve, _reject) => {
	console.log('test'); // immediately executed in the constructor
	resolve(); // yields back to the main loop to keep going through the call stack
})

thing.then( () => {
	console.log("test2"); // this ends up added to the call stack before the main loops callback
} )
