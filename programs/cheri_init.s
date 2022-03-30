# RISC-V baremetal init.s
# This code is executed first.

.section .text.init
entry:
	#la t0, __sp-32           # load stack pointer address into register
	#CSpecialRW csp, ddc, c0       # Load default-data-capability into stack pointer
	#CSetAddr  csp, csp, t0   # set stack pointer address within ddc, using a constant defined in the linker script.

	#la    sp, __sp-32   # set up the stack pointer, using a constant defined in the linker script. (no-capability version)
	clgc csp, __sp       # cheri version (using the -32 causes a "capability table entry relocs should not have addends" failure)

	#call  main          # call the main function
	clgc ct1, main       # get 'main' symbol from global table
	cjalr cra, ct1       # jump-and-link to it 

end:
	j end               # loop when finished if there is no environment to return to.