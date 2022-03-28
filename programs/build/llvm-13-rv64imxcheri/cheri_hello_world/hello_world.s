	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p0_m2p0_v0p10_zvlsseg0p10_xcheri0p0"
	.file	"hello_world.c"
	.globl	factorial
	.p2align	2
	.type	factorial,@function
factorial:
	cincoffset	csp, csp, -32
	csc	cra, 16(csp)
	csc	cs0, 0(csp)
	beqz	a0, .LBB0_2
	mv	s0, a0
	addiw	a0, a0, -1
	ccall	factorial
	mulw	a0, a0, s0
	j	.LBB0_3
.LBB0_2:
	addi	a0, zero, 1
.LBB0_3:
	clc	cs0, 0(csp)
	clc	cra, 16(csp)
	cincoffset	csp, csp, 32
	cret
.Lfunc_end0:
	.size	factorial, .Lfunc_end0-factorial

	.globl	fac_test
	.p2align	2
	.type	fac_test,@function
fac_test:
	cincoffset	csp, csp, -16
	csc	cra, 0(csp)
	addi	a0, zero, 10
	ccall	factorial
	lui	a1, 886
	addiw	a1, a1, -256
	xor	a0, a0, a1
	seqz	a0, a0
	clc	cra, 0(csp)
	cincoffset	csp, csp, 16
	cret
.Lfunc_end1:
	.size	fac_test, .Lfunc_end1-fac_test

	.globl	fibbonacci
	.p2align	2
	.type	fibbonacci,@function
fibbonacci:
	cincoffset	csp, csp, -48
	csc	cra, 32(csp)
	csc	cs0, 16(csp)
	csc	cs1, 0(csp)
	mv	s0, a0
	beqz	a0, .LBB2_3
	addi	a0, zero, 1
	beq	s0, a0, .LBB2_4
	addiw	a0, s0, -1
	ccall	fibbonacci
	mv	s1, a0
	addiw	a0, s0, -2
	ccall	fibbonacci
	addw	a0, a0, s1
	j	.LBB2_4
.LBB2_3:
	mv	a0, s0
.LBB2_4:
	clc	cs1, 0(csp)
	clc	cs0, 16(csp)
	clc	cra, 32(csp)
	cincoffset	csp, csp, 48
	cret
.Lfunc_end2:
	.size	fibbonacci, .Lfunc_end2-fibbonacci

	.globl	fib_test
	.p2align	2
	.type	fib_test,@function
fib_test:
	cincoffset	csp, csp, -16
	csc	cra, 0(csp)
	addi	a0, zero, 10
	ccall	fibbonacci
	addi	a0, a0, -55
	seqz	a0, a0
	clc	cra, 0(csp)
	cincoffset	csp, csp, 16
	cret
.Lfunc_end3:
	.size	fib_test, .Lfunc_end3-fib_test

	.globl	main
	.p2align	2
	.type	main,@function
main:
	cincoffset	csp, csp, -32
	csc	cra, 16(csp)
	csc	cs0, 0(csp)
	ccall	fac_test
	mv	s0, a0
	ccall	fib_test
	slli	a0, a0, 1
	or	a1, a0, s0
	addi	a0, zero, 15
	slli	a0, a0, 28
	cincoffset	ca2, cnull, a0
	sext.w	a0, a1
	csw	a1, 0(ca2)
	clc	cs0, 0(csp)
	clc	cra, 16(csp)
	cincoffset	csp, csp, 32
	cret
.Lfunc_end4:
	.size	main, .Lfunc_end4-main

	.ident	"clang version 13.0.0 (ssh://git@github.com/theturboturnip/llvm-project.git 62cac4e2d70fb43bf3bef79e2f3821a5c1805588)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
