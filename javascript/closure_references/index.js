// using a builder patter for DI works really well, but closures can cause memory leaks
function buildMain(dep) {
	// here we add a dep and return a function that will hold that reference using lexical scoping
	return function () {
	}
}

let dep = {name: 'test', thing: {test: 'thing'}};
while (true) {

// pass in the dep
const main = buildMain(dep);

// print it
main();
// its the same dep as before!

dep['desc'] = "hey im main";

// mutating the object works, because its the same reference
main();

// mutating objects in the object is fine
dep.thing = {...dep.thing, mutated: true}
main();

// reassigning that reference does not work.
// this creates a new obj in memory and assigns the reference to that to our dep variable

dep = {name: 'not test'};

// but the main function still holds a reference to the original data.
// this means that even though in our outer most scope, we reassigned the variable
// the original allocated data is still referenced in our main function and GC will not clean it up
// if you build and forget a lot of functions via closures, you can end up with a huge memory leak
main();
}
