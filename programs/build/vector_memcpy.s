	.text
	.attribute	4, 16
	.attribute	5, "rv32i2p0_m2p0_a2p0_f2p0_d2p0_v0p10_zvlsseg0p10"
	.file	"vector_memcpy.c"
	.globl	fib
	.p2align	2
	.type	fib,@function
fib:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	sw	s0, 8(sp)
	sw	s1, 4(sp)
	addi	a1, zero, 2
	mv	s0, a0
	blt	a0, a1, .LBB0_2
	addi	a0, s0, -1
	call	fib
	mv	s1, a0
	addi	a0, s0, -2
	call	fib
	add	s0, a0, s1
.LBB0_2:
	mv	a0, s0
	lw	s1, 4(sp)
	lw	s0, 8(sp)
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end0:
	.size	fib, .Lfunc_end0-fib

	.globl	main
	.p2align	2
	.type	main,@function
main:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	addi	a0, zero, 8
	call	fib
	lui	a1, 983040
	sw	a0, 0(a1)
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end1:
	.size	main, .Lfunc_end1-main

	.ident	"Ubuntu clang version 13.0.1-++20211215022811+5932c004778c-1~exp1~20211215022819.27"
	.section	".note.GNU-stack","",@progbits
	.addrsig
