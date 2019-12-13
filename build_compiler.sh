#!/bin/fish
cd /home/jan/uni/PdF/compiler/rustc
printf "\n\n\n---------------------------------------------\nStart Checking\n---------------------------------------------\n\n\n"
./x.py check
if test $status = 0
	printf "\n\n\n---------------------------------------------\nStart building\n---------------------------------------------\n\n\n"
	./x.py -i build --stage 1 src/libstd --keep-stage 1
	# der folgende befehl ist zwar der richtige ansatz aber so würde der stage 1 compiler nicht gebaut werden
	#/home/jan/Uni/PdF/rustc/x.py build -i --stage 1 --keep-stage 1
	exit $status
else
	printf "\n\n\n---------------------------------------------\nAbort building\n---------------------------------------------\n\n\n"
	exit 1
end
