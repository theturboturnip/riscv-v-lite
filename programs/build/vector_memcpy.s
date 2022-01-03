	.text
	.attribute	4, 16
	.attribute	5, "rv32i2p0_m2p0_a2p0_f2p0_d2p0_v0p10_zvlsseg0p10"
	.file	"vector_memcpy.c"
	.globl	vector_memcpy_e8
	.p2align	2
	.type	vector_memcpy_e8,@function
vector_memcpy_e8:
	beqz	a0, .LBB0_2
.LBB0_1:
	vsetvli	a3, a0, e32, m8, ta, mu
	vle32.v	v8, (a1)
	vse32.v	v8, (a2)
	slli	a4, a3, 2
	add	a1, a1, a4
	sub	a0, a0, a3
	add	a2, a2, a4
	bnez	a0, .LBB0_1
.LBB0_2:
	ret
.Lfunc_end0:
	.size	vector_memcpy_e8, .Lfunc_end0-vector_memcpy_e8

	.globl	vector_memcpy_harness
	.p2align	2
	.type	vector_memcpy_harness,@function
vector_memcpy_harness:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	sw	s0, 8(sp)
	sw	s1, 4(sp)
	sw	s2, 0(sp)
	mv	a0, zero
	lui	a1, %hi(vector_memcpy_harness.data)
	addi	a1, a1, %lo(vector_memcpy_harness.data)
	lui	a2, 1
	addi	a2, a2, -2048
.LBB1_1:
	sw	a0, 0(a1)
	addi	a0, a0, 1
	addi	a1, a1, 4
	bne	a0, a2, .LBB1_1
	lui	a0, %hi(vector_memcpy_harness.data)
	addi	s2, a0, %lo(vector_memcpy_harness.data)
	lui	a0, %hi(vector_memcpy_harness.out_data)
	addi	s1, a0, %lo(vector_memcpy_harness.out_data)
	addi	a0, zero, 1003
	addi	s0, zero, 1003
	mv	a1, s2
	mv	a2, s1
	call	vector_memcpy_e8
.LBB1_3:
	lw	a0, 0(s2)
	lw	a1, 0(s1)
	bne	a0, a1, .LBB1_10
	addi	s0, s0, -1
	addi	s1, s1, 4
	addi	s2, s2, 4
	bnez	s0, .LBB1_3
	lui	a0, %hi(vector_memcpy_harness.out_data)
	addi	a1, a0, %lo(vector_memcpy_harness.out_data)
	addi	a0, a1, 2006
	lw	a2, 2006(a0)
	mv	a0, zero
	bnez	a2, .LBB1_11
	addi	a3, zero, 1003
	addi	a0, a1, 2008
	addi	a0, a0, 2008
	addi	a1, zero, 2047
.LBB1_7:
	mv	a2, a3
	beq	a3, a1, .LBB1_9
	lw	a4, 0(a0)
	addi	a0, a0, 4
	addi	a3, a2, 1
	beqz	a4, .LBB1_7
.LBB1_9:
	addi	a0, zero, 2046
	sltu	a0, a0, a2
	j	.LBB1_11
.LBB1_10:
	mv	a0, zero
.LBB1_11:
	lw	s2, 0(sp)
	lw	s1, 4(sp)
	lw	s0, 8(sp)
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end1:
	.size	vector_memcpy_harness, .Lfunc_end1-vector_memcpy_harness

	.globl	main
	.p2align	2
	.type	main,@function
main:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	call	vector_memcpy_harness
	lui	a1, 983040
	sw	a0, 0(a1)
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end2:
	.size	main, .Lfunc_end2-main

	.type	vector_memcpy_harness.data,@object
	.local	vector_memcpy_harness.data
	.comm	vector_memcpy_harness.data,8192,4
	.type	vector_memcpy_harness.out_data,@object
	.local	vector_memcpy_harness.out_data
	.comm	vector_memcpy_harness.out_data,8192,4
	.ident	"Ubuntu clang version 13.0.1-++20211215022811+5932c004778c-1~exp1~20211215022819.27"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym vector_memcpy_harness.data
	.addrsig_sym vector_memcpy_harness.out_data
