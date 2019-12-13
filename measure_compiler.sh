#!/bin/fish
./build_compiler.sh
if test $status = 0
	if test (count $argv) -gt 0
		switch $argv[1]
			case hello_world
				set name hello_world
			case fibonacci
				set name fibonacci
			case vec_test
				set name vec_test
			case slice_test
				set name slice_test
			case '*'
				echo unkown command
				exit 1
		end
	else
		#no args => run Hello World test
		set name hello_world
	end

	rustc +stage1 -O -Z remove_bound_checks -Z dump-mir=all -Z dump-mir-graphviz -Z dump-mir-dir=~/uni/PdF/test/mir/$name-without-bound-checks/ -o ~/uni/PdF/test/executables/$name-without-bound-checks ~/uni/PdF/test/test_programms/$name/src/main.rs 
	rustc +stage1 -O -Z dump-mir=all -Z dump-mir-graphviz -Z dump-mir-dir=~/uni/PdF/test/mir/$name/ -o ~/uni/PdF/test/executables/$name ~/uni/PdF/test/test_programms/$name/src/main.rs 

	printf "\n\n\n---------------------------------------------\nMessure Performance\n---------------------------------------------\n\n\n"

	hyperfine -w 10 -r 1000 ~/uni/PdF/test/executables/$name-without-bound-checks ~/uni/PdF/test/executables/$name
else
	echo could not bulid compiler
	exit 1
end
