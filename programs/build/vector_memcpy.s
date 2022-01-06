	.text
	.attribute	4, 16
	.attribute	5, "rv32i2p0_m2p0_a2p0_f2p0_d2p0_v0p10_zvlsseg0p10"
	.file	"vector_memcpy.c"
	.globl	memset
	.p2align	2
	.type	memset,@function
memset:
	beqz	a2, .LBB0_3
	mv	a3, a0
.LBB0_2:
	sb	a1, 0(a3)
	addi	a2, a2, -1
	addi	a3, a3, 1
	bnez	a2, .LBB0_2
.LBB0_3:
	ret
.Lfunc_end0:
	.size	memset, .Lfunc_end0-memset

	.globl	vector_memcpy_32mf2
	.p2align	2
	.type	vector_memcpy_32mf2,@function
vector_memcpy_32mf2:
	beqz	a0, .LBB1_2
.LBB1_1:
	vsetvli	a3, a0, e32, mf2, ta, mu
	vle32.v	v25, (a1)
	vse32.v	v25, (a2)
	slli	a4, a3, 2
	add	a1, a1, a4
	sub	a0, a0, a3
	add	a2, a2, a4
	bnez	a0, .LBB1_1
.LBB1_2:
	ret
.Lfunc_end1:
	.size	vector_memcpy_32mf2, .Lfunc_end1-vector_memcpy_32mf2

	.globl	vector_memcpy_32m8
	.p2align	2
	.type	vector_memcpy_32m8,@function
vector_memcpy_32m8:
	beqz	a0, .LBB2_2
.LBB2_1:
	vsetvli	a3, a0, e32, m8, ta, mu
	vle32.v	v8, (a1)
	vse32.v	v8, (a2)
	slli	a4, a3, 2
	add	a1, a1, a4
	sub	a0, a0, a3
	add	a2, a2, a4
	bnez	a0, .LBB2_1
.LBB2_2:
	ret
.Lfunc_end2:
	.size	vector_memcpy_32m8, .Lfunc_end2-vector_memcpy_32m8

	.globl	vector_memcpy_harness
	.p2align	2
	.type	vector_memcpy_harness,@function
vector_memcpy_harness:
	addi	sp, sp, -2032
	sw	ra, 2028(sp)
	sw	s0, 2024(sp)
	sw	s1, 2020(sp)
	sw	s2, 2016(sp)
	sw	s3, 2012(sp)
	lui	a1, 4
	addi	a1, a1, -2000
	sub	sp, sp, a1
	mv	s2, a0
	lui	a0, 2
	addi	a0, a0, 12
	add	a0, sp, a0
	lui	a2, 2
	lui	a1, 2
	addi	a1, a1, 12
	add	s0, sp, a1
	mv	a1, zero
	call	memset@plt
	addi	a0, sp, 12
	lui	a2, 2
	mv	a1, zero
	call	memset@plt
	mv	a0, zero
	lui	a1, 1
	addi	a1, a1, -2048
.LBB3_1:
	sw	a0, 0(s0)
	addi	a0, a0, 1
	addi	s0, s0, 4
	bne	a0, a1, .LBB3_1
	addi	s3, zero, 1003
	lui	a0, 2
	addi	a0, a0, 12
	add	s0, sp, a0
	addi	a0, zero, 1003
	lui	a1, 2
	addi	a1, a1, 12
	add	a1, sp, a1
	addi	a2, sp, 12
	addi	s1, sp, 12
	jalr	s2
.LBB3_3:
	lw	a0, 0(s0)
	lw	a1, 0(s1)
	bne	a0, a1, .LBB3_10
	addi	s3, s3, -1
	addi	s1, s1, 4
	addi	s0, s0, 4
	bnez	s3, .LBB3_3
	addi	a1, sp, 12
	addi	a0, a1, 2006
	lw	a2, 2006(a0)
	mv	a0, zero
	bnez	a2, .LBB3_11
	addi	a0, a1, 2008
	addi	a0, a0, 2008
	addi	a3, zero, 1003
	addi	a1, zero, 2047
.LBB3_7:
	mv	a2, a3
	beq	a3, a1, .LBB3_9
	lw	a4, 0(a0)
	addi	a0, a0, 4
	addi	a3, a2, 1
	beqz	a4, .LBB3_7
.LBB3_9:
	addi	a0, zero, 2046
	sltu	a0, a0, a2
	j	.LBB3_11
.LBB3_10:
	mv	a0, zero
.LBB3_11:
	lui	a1, 4
	addi	a1, a1, -2000
	add	sp, sp, a1
	lw	s3, 2012(sp)
	lw	s2, 2016(sp)
	lw	s1, 2020(sp)
	lw	s0, 2024(sp)
	lw	ra, 2028(sp)
	addi	sp, sp, 2032
	ret
.Lfunc_end3:
	.size	vector_memcpy_harness, .Lfunc_end3-vector_memcpy_harness

	.globl	main
	.p2align	2
	.type	main,@function
main:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	sw	s0, 8(sp)
	lui	a0, %hi(vector_memcpy_32m8)
	addi	a0, a0, %lo(vector_memcpy_32m8)
	call	vector_memcpy_harness
	mv	s0, a0
	lui	a0, %hi(vector_memcpy_32mf2)
	addi	a0, a0, %lo(vector_memcpy_32mf2)
	call	vector_memcpy_harness
	slli	a0, a0, 1
	or	a0, a0, s0
	lui	a1, 983040
	sw	a0, 0(a1)
	lw	s0, 8(sp)
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end4:
	.size	main, .Lfunc_end4-main

	.ident	"Ubuntu clang version 13.0.1-++20211215022811+5932c004778c-1~exp1~20211215022819.27"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym vector_memcpy_32mf2
	.addrsig_sym vector_memcpy_32m8
