	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p0_xcheri0p0"
	.file	"hello_world.c"
	.globl	main
	.p2align	2
	.type	main,@function
main:
	addi	a0, zero, 15
	slli	a0, a0, 28
	cincoffset	ca1, cnull, a0
	addi	a2, zero, 1
	addi	a0, zero, 1
	csw	a2, 0(ca1)
	cret
.Lfunc_end0:
	.size	main, .Lfunc_end0-main

	.ident	"clang version 13.0.0 (https://github.com/CTSRD-CHERI/llvm-project.git 7245208df70403e1f7189f3be7b57ca934b1cbb2)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
