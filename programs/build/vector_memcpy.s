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
	addi	sp, sp, -32
	sw	ra, 28(sp)
	sw	s0, 24(sp)
	sw	s1, 20(sp)
	sw	s2, 16(sp)
	sw	s3, 12(sp)
	mv	a0, zero
	lui	a1, %hi(vector_memcpy_harness.data)
	addi	a1, a1, %lo(vector_memcpy_harness.data)
	addi	a2, zero, 1003
.LBB1_1:
	sw	a0, 0(a1)
	addi	a0, a0, 1
	addi	a1, a1, 4
	bne	a0, a2, .LBB1_1
	lui	s2, %hi(vector_memcpy_harness.data)
	addi	s0, s2, %lo(vector_memcpy_harness.data)
	lui	s3, %hi(vector_memcpy_harness.out_data)
	addi	s1, s3, %lo(vector_memcpy_harness.out_data)
	addi	a0, zero, 1003
	mv	a1, s0
	mv	a2, s1
	call	vector_memcpy_e8
	lw	a0, %lo(vector_memcpy_harness.data)(s2)
	lw	a1, %lo(vector_memcpy_harness.out_data)(s3)
	bne	a0, a1, .LBB1_7
	mv	a4, zero
	addi	a0, s0, 4
	addi	a1, s1, 4
	addi	a3, zero, 1002
.LBB1_4:
	mv	a2, a4
	beq	a4, a3, .LBB1_6
	lw	a5, 0(a0)
	lw	s1, 0(a1)
	addi	a0, a0, 4
	addi	a1, a1, 4
	addi	a4, a2, 1
	beq	a5, s1, .LBB1_4
.LBB1_6:
	addi	a0, zero, 1001
	sltu	a0, a0, a2
	j	.LBB1_8
.LBB1_7:
	mv	a0, zero
.LBB1_8:
	lw	s3, 12(sp)
	lw	s2, 16(sp)
	lw	s1, 20(sp)
	lw	s0, 24(sp)
	lw	ra, 28(sp)
	addi	sp, sp, 32
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
	.comm	vector_memcpy_harness.data,4012,4
	.type	vector_memcpy_harness.out_data,@object
	.local	vector_memcpy_harness.out_data
	.comm	vector_memcpy_harness.out_data,4012,4
	.ident	"Ubuntu clang version 13.0.1-++20211215022811+5932c004778c-1~exp1~20211215022819.27"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym vector_memcpy_harness.data
	.addrsig_sym vector_memcpy_harness.out_data
