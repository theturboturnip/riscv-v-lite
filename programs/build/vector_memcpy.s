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

	.globl	vector_memcpy_indexed
	.p2align	2
	.type	vector_memcpy_indexed,@function
vector_memcpy_indexed:
	addi	sp, sp, -544
	sw	ra, 540(sp)
	sw	s0, 536(sp)
	sw	s1, 532(sp)
	sw	s2, 528(sp)
	sw	s3, 524(sp)
	mv	s2, a2
	mv	s3, a1
	mv	s0, a0
	addi	a0, sp, 12
	addi	a2, zero, 512
	addi	s1, sp, 12
	mv	a1, zero
	call	memset@plt
	vsetvli	a0, zero, e32, m4, ta, mu
	beqz	a0, .LBB1_3
	mv	a1, zero
.LBB1_2:
	xori	a2, a1, 1
	sw	a2, 0(s1)
	addi	a1, a1, 1
	addi	s1, s1, 4
	bne	a0, a1, .LBB1_2
.LBB1_3:
	addi	a1, sp, 12
	vle32.v	v28, (a1)
	bnez	s0, .LBB1_7
.LBB1_4:
	lw	s3, 524(sp)
	lw	s2, 528(sp)
	lw	s1, 532(sp)
	lw	s0, 536(sp)
	lw	ra, 540(sp)
	addi	sp, sp, 544
	ret
.LBB1_5:
	vle32.v	v8, (s3)
	vse32.v	v8, (s2)
.LBB1_6:
	slli	a2, a1, 2
	add	s3, s3, a2
	sub	s0, s0, a1
	add	s2, s2, a2
	beqz	s0, .LBB1_4
.LBB1_7:
	vsetvli	a1, s0, e32, m4, ta, mu
	bne	a1, a0, .LBB1_5
	vloxei32.v	v8, (s3), v28
	vsoxei32.v	v8, (s2), v28
	j	.LBB1_6
.Lfunc_end1:
	.size	vector_memcpy_indexed, .Lfunc_end1-vector_memcpy_indexed

	.globl	vector_memcpy_masked
	.p2align	2
	.type	vector_memcpy_masked,@function
vector_memcpy_masked:
	addi	sp, sp, -544
	sw	ra, 540(sp)
	sw	s0, 536(sp)
	sw	s1, 532(sp)
	sw	s2, 528(sp)
	sw	s3, 524(sp)
	mv	s2, a2
	mv	s3, a1
	mv	s0, a0
	addi	a0, sp, 12
	addi	a2, zero, 512
	addi	s1, sp, 12
	mv	a1, zero
	call	memset@plt
	vsetvli	a0, zero, e32, m4, ta, mu
	beqz	a0, .LBB2_3
	mv	a1, zero
.LBB2_2:
	andi	a2, a1, 1
	sw	a2, 0(s1)
	addi	a1, a1, 1
	addi	s1, s1, 4
	bne	a0, a1, .LBB2_2
.LBB2_3:
	addi	a0, sp, 12
	vle32.v	v28, (a0)
	beqz	s0, .LBB2_6
	vmseq.vi	v0, v28, 1
	vmv.v.i	v28, 0
.LBB2_5:
	vsetvli	a0, s0, e32, m4, ta, mu
	vsetvli	zero, a0, e32, m4, tu, mu
	vmv4r.v	v8, v28
	vle32.v	v8, (s3), v0.t
	vsetvli	zero, zero, e32, m4, ta, mu
	vse32.v	v8, (s2), v0.t
	slli	a1, a0, 2
	add	s3, s3, a1
	sub	s0, s0, a0
	add	s2, s2, a1
	bnez	s0, .LBB2_5
.LBB2_6:
	lw	s3, 524(sp)
	lw	s2, 528(sp)
	lw	s1, 532(sp)
	lw	s0, 536(sp)
	lw	ra, 540(sp)
	addi	sp, sp, 544
	ret
.Lfunc_end2:
	.size	vector_memcpy_masked, .Lfunc_end2-vector_memcpy_masked

	.globl	vector_memcpy_8strided
	.p2align	2
	.type	vector_memcpy_8strided,@function
vector_memcpy_8strided:
	beqz	a0, .LBB3_7
	lui	a3, 262144
	addi	a6, a3, -1
	addi	t1, zero, 4
	addi	t0, zero, 16
	j	.LBB3_4
.LBB3_2:
	vle8.v	v25, (a1)
	vse8.v	v25, (a2)
	srli	a7, a3, 2
.LBB3_3:
	sub	a0, a0, a7
	slli	a3, a7, 2
	add	a1, a1, a3
	add	a2, a2, a3
	beqz	a0, .LBB3_7
.LBB3_4:
	slli	a3, a0, 2
	vsetvli	a3, a3, e8, m1, ta, mu
	and	a7, a3, a6
	bgeu	a7, a0, .LBB3_2
	mv	a5, zero
.LBB3_6:
	add	a4, a1, a5
	vsetvli	zero, a3, e8, m1, ta, mu
	vlse8.v	v25, (a4), t1
	add	a4, a2, a5
	addi	a5, a5, 1
	vsse8.v	v25, (a4), t1
	bne	a5, t0, .LBB3_6
	j	.LBB3_3
.LBB3_7:
	ret
.Lfunc_end3:
	.size	vector_memcpy_8strided, .Lfunc_end3-vector_memcpy_8strided

	.globl	vector_memcpy_16strided
	.p2align	2
	.type	vector_memcpy_16strided,@function
vector_memcpy_16strided:
	beqz	a0, .LBB4_9
	mv	a3, zero
	addi	a6, zero, 1
	addi	t1, zero, 4
	addi	t0, zero, 16
	j	.LBB4_4
.LBB4_2:
	vle16.v	v25, (a1)
	vse16.v	v25, (a2)
	srli	a5, a3, 1
.LBB4_3:
	sub	a0, a0, a5
	slli	a4, a5, 2
	add	a1, a1, a4
	add	a2, a2, a4
	beqz	a0, .LBB4_9
.LBB4_4:
	addi	a5, a0, -1
	snez	a5, a5
	sltu	a3, a6, a3
	or	a3, a5, a3
	beqz	a3, .LBB4_9
	slli	a3, a0, 1
	vsetvli	a3, a3, e16, m1, ta, mu
	slli	a7, a3, 2
	bgeu	a7, a0, .LBB4_2
	mv	a5, zero
.LBB4_7:
	add	a4, a1, a5
	vsetvli	zero, a3, e16, m1, ta, mu
	vlse16.v	v25, (a4), t1
	add	a4, a2, a5
	addi	a5, a5, 2
	vsse16.v	v25, (a4), t1
	bne	a5, t0, .LBB4_7
	srli	a5, a7, 1
	j	.LBB4_3
.LBB4_9:
	ret
.Lfunc_end4:
	.size	vector_memcpy_16strided, .Lfunc_end4-vector_memcpy_16strided

	.globl	vector_memcpy_32strided
	.p2align	2
	.type	vector_memcpy_32strided,@function
vector_memcpy_32strided:
	beqz	a0, .LBB5_8
	addi	t0, zero, 4
	addi	a7, zero, 16
	j	.LBB5_4
.LBB5_2:
	vle32.v	v25, (a1)
	vse32.v	v25, (a2)
.LBB5_3:
	sub	a0, a0, a5
	slli	a3, a5, 2
	add	a1, a1, a3
	add	a2, a2, a3
	beqz	a0, .LBB5_8
.LBB5_4:
	vsetvli	a5, a0, e32, m1, ta, mu
	slli	a6, a5, 2
	bgeu	a6, a0, .LBB5_2
	mv	a4, zero
.LBB5_6:
	add	a3, a1, a4
	vsetvli	zero, a5, e32, m1, ta, mu
	vlse32.v	v25, (a3), t0
	add	a3, a2, a4
	addi	a4, a4, 4
	vsse32.v	v25, (a3), t0
	bne	a4, a7, .LBB5_6
	mv	a5, a6
	j	.LBB5_3
.LBB5_8:
	ret
.Lfunc_end5:
	.size	vector_memcpy_32strided, .Lfunc_end5-vector_memcpy_32strided

	.globl	vector_memcpy_32mf2
	.p2align	2
	.type	vector_memcpy_32mf2,@function
vector_memcpy_32mf2:
	beqz	a0, .LBB6_2
.LBB6_1:
	vsetvli	a3, a0, e32, mf2, ta, mu
	vle32.v	v25, (a1)
	vse32.v	v25, (a2)
	slli	a4, a3, 2
	add	a1, a1, a4
	sub	a0, a0, a3
	add	a2, a2, a4
	bnez	a0, .LBB6_1
.LBB6_2:
	ret
.Lfunc_end6:
	.size	vector_memcpy_32mf2, .Lfunc_end6-vector_memcpy_32mf2

	.globl	vector_memcpy_8m8
	.p2align	2
	.type	vector_memcpy_8m8,@function
vector_memcpy_8m8:
	beqz	a0, .LBB7_4
	mv	a4, zero
	addi	a6, zero, 3
.LBB7_2:
	addi	a5, a0, -1
	snez	a5, a5
	sltu	a4, a6, a4
	or	a4, a5, a4
	beqz	a4, .LBB7_4
	slli	a4, a0, 2
	vsetvli	a4, a4, e8, m8, ta, mu
	vle8.v	v8, (a1)
	vse8.v	v8, (a2)
	srli	a5, a4, 2
	slli	a3, a5, 2
	add	a1, a1, a3
	sub	a0, a0, a5
	add	a2, a2, a3
	bnez	a0, .LBB7_2
.LBB7_4:
	ret
.Lfunc_end7:
	.size	vector_memcpy_8m8, .Lfunc_end7-vector_memcpy_8m8

	.globl	vector_memcpy_16m8
	.p2align	2
	.type	vector_memcpy_16m8,@function
vector_memcpy_16m8:
	beqz	a0, .LBB8_4
	mv	a4, zero
	addi	a6, zero, 1
.LBB8_2:
	addi	a5, a0, -1
	snez	a5, a5
	sltu	a4, a6, a4
	or	a4, a5, a4
	beqz	a4, .LBB8_4
	slli	a4, a0, 1
	vsetvli	a4, a4, e16, m8, ta, mu
	vle16.v	v8, (a1)
	vse16.v	v8, (a2)
	srli	a5, a4, 1
	slli	a3, a5, 2
	add	a1, a1, a3
	sub	a0, a0, a5
	add	a2, a2, a3
	bnez	a0, .LBB8_2
.LBB8_4:
	ret
.Lfunc_end8:
	.size	vector_memcpy_16m8, .Lfunc_end8-vector_memcpy_16m8

	.globl	vector_memcpy_32m8
	.p2align	2
	.type	vector_memcpy_32m8,@function
vector_memcpy_32m8:
	beqz	a0, .LBB9_2
.LBB9_1:
	vsetvli	a3, a0, e32, m8, ta, mu
	vle32.v	v8, (a1)
	vse32.v	v8, (a2)
	slli	a4, a3, 2
	add	a1, a1, a4
	sub	a0, a0, a3
	add	a2, a2, a4
	bnez	a0, .LBB9_1
.LBB9_2:
	ret
.Lfunc_end9:
	.size	vector_memcpy_32m8, .Lfunc_end9-vector_memcpy_32m8

	.globl	vector_memcpy_32m2_seg4load
	.p2align	2
	.type	vector_memcpy_32m2_seg4load,@function
vector_memcpy_32m2_seg4load:
	beqz	a0, .LBB10_5
	addi	a7, zero, 16
.LBB10_2:
	vsetvli	a6, a0, e32, m2, ta, mu
	vlseg4e32.v	v0, (a1)
	lw	a4, 0(a2)
	vse32.v	v0, (a4)
	lw	a4, 4(a2)
	vse32.v	v2, (a4)
	lw	a4, 8(a2)
	vse32.v	v4, (a4)
	lw	a5, 12(a2)
	mv	a4, zero
	vse32.v	v6, (a5)
	slli	t0, a6, 2
.LBB10_3:
	add	a3, a2, a4
	lw	a5, 0(a3)
	add	a5, a5, t0
	addi	a4, a4, 4
	sw	a5, 0(a3)
	bne	a4, a7, .LBB10_3
	slli	a4, t0, 2
	sub	a0, a0, a6
	add	a1, a1, a4
	bnez	a0, .LBB10_2
.LBB10_5:
	ret
.Lfunc_end10:
	.size	vector_memcpy_32m2_seg4load, .Lfunc_end10-vector_memcpy_32m2_seg4load

	.globl	vector_memcpy_harness
	.p2align	2
	.type	vector_memcpy_harness,@function
vector_memcpy_harness:
	addi	sp, sp, -1056
	sw	ra, 1052(sp)
	sw	s0, 1048(sp)
	sw	s1, 1044(sp)
	sw	s2, 1040(sp)
	mv	s2, a0
	addi	a0, sp, 528
	addi	a2, zero, 512
	addi	s1, sp, 528
	mv	a1, zero
	call	memset@plt
	addi	a0, sp, 16
	addi	a2, zero, 512
	mv	a1, zero
	call	memset@plt
	mv	a0, zero
	addi	a1, zero, 128
.LBB11_1:
	sw	a0, 0(s1)
	addi	a0, a0, 1
	addi	s1, s1, 4
	bne	a0, a1, .LBB11_1
	addi	s1, sp, 528
	addi	a0, zero, 103
	addi	a1, sp, 528
	addi	a2, sp, 16
	addi	s0, sp, 16
	jalr	s2
	mv	a0, zero
	addi	a1, zero, 412
.LBB11_3:
	add	a2, s1, a0
	lw	a2, 0(a2)
	add	a3, s0, a0
	lw	a3, 0(a3)
	bne	a2, a3, .LBB11_10
	addi	a0, a0, 4
	bne	a0, a1, .LBB11_3
	lw	a1, 428(sp)
	mv	a0, zero
	bnez	a1, .LBB11_11
	addi	a0, sp, 432
	addi	a3, zero, 103
	addi	a1, zero, 127
.LBB11_7:
	mv	a2, a3
	beq	a3, a1, .LBB11_9
	lw	a4, 0(a0)
	addi	a0, a0, 4
	addi	a3, a2, 1
	beqz	a4, .LBB11_7
.LBB11_9:
	addi	a0, zero, 126
	sltu	a0, a0, a2
	j	.LBB11_11
.LBB11_10:
	mv	a0, zero
.LBB11_11:
	lw	s2, 1040(sp)
	lw	s1, 1044(sp)
	lw	s0, 1048(sp)
	lw	ra, 1052(sp)
	addi	sp, sp, 1056
	ret
.Lfunc_end11:
	.size	vector_memcpy_harness, .Lfunc_end11-vector_memcpy_harness

	.globl	vector_memcpy_masked_harness
	.p2align	2
	.type	vector_memcpy_masked_harness,@function
vector_memcpy_masked_harness:
	addi	sp, sp, -1056
	sw	ra, 1052(sp)
	sw	s0, 1048(sp)
	sw	s1, 1044(sp)
	sw	s2, 1040(sp)
	sw	s3, 1036(sp)
	mv	s2, a0
	addi	a0, sp, 524
	addi	a2, zero, 512
	addi	s0, sp, 524
	mv	a1, zero
	call	memset@plt
	addi	a0, sp, 12
	addi	a2, zero, 512
	mv	a1, zero
	call	memset@plt
	addi	a0, sp, 12
	addi	a1, zero, 255
	addi	a2, zero, 512
	call	memset
	mv	a0, zero
	addi	a1, zero, 128
.LBB12_1:
	sw	a0, 0(s0)
	addi	a0, a0, 1
	addi	s0, s0, 4
	bne	a0, a1, .LBB12_1
	addi	s3, zero, 103
	addi	s0, sp, 524
	addi	a0, zero, 103
	addi	a1, sp, 524
	addi	a2, sp, 12
	addi	s1, sp, 12
	jalr	s2
	mv	a0, zero
	addi	a1, zero, -1
	j	.LBB12_5
.LBB12_3:
	lw	a2, 0(s0)
	lw	a3, 0(s1)
	bne	a2, a3, .LBB12_8
.LBB12_4:
	addi	a0, a0, 1
	addi	s1, s1, 4
	addi	s0, s0, 4
	beq	a0, s3, .LBB12_7
.LBB12_5:
	andi	a2, a0, 1
	bnez	a2, .LBB12_3
	lw	a2, 0(s1)
	beq	a2, a1, .LBB12_4
	j	.LBB12_8
.LBB12_7:
	lw	a1, 424(sp)
	addi	a0, zero, -1
	beq	a1, a0, .LBB12_10
.LBB12_8:
	mv	a0, zero
.LBB12_9:
	lw	s3, 1036(sp)
	lw	s2, 1040(sp)
	lw	s1, 1044(sp)
	lw	s0, 1048(sp)
	lw	ra, 1052(sp)
	addi	sp, sp, 1056
	ret
.LBB12_10:
	addi	a1, sp, 428
	addi	a4, zero, 103
	addi	a2, zero, 127
.LBB12_11:
	mv	a3, a4
	beq	a4, a2, .LBB12_13
	lw	a5, 0(a1)
	addi	a1, a1, 4
	addi	a4, a3, 1
	beq	a5, a0, .LBB12_11
.LBB12_13:
	addi	a0, zero, 126
	sltu	a0, a0, a3
	j	.LBB12_9
.Lfunc_end12:
	.size	vector_memcpy_masked_harness, .Lfunc_end12-vector_memcpy_masked_harness

	.globl	vector_memcpy_segmented_harness_i32
	.p2align	2
	.type	vector_memcpy_segmented_harness_i32,@function
vector_memcpy_segmented_harness_i32:
	addi	sp, sp, -1072
	sw	ra, 1068(sp)
	sw	s0, 1064(sp)
	sw	s1, 1060(sp)
	sw	s2, 1056(sp)
	sw	s3, 1052(sp)
	sw	s4, 1048(sp)
	mv	s2, a0
	addi	a0, sp, 536
	addi	a2, zero, 512
	addi	s0, sp, 536
	mv	a1, zero
	call	memset@plt
	addi	a0, sp, 408
	addi	a2, zero, 128
	addi	s1, zero, 128
	mv	a1, zero
	call	memset@plt
	addi	a0, sp, 280
	addi	a2, zero, 128
	mv	a1, zero
	call	memset@plt
	addi	a0, sp, 152
	addi	a2, zero, 128
	mv	a1, zero
	call	memset@plt
	addi	a0, sp, 24
	addi	a2, zero, 128
	mv	a1, zero
	call	memset@plt
	mv	a0, zero
.LBB13_1:
	sw	a0, 0(s0)
	addi	a0, a0, 1
	addi	s0, s0, 4
	bne	a0, s1, .LBB13_1
	addi	s3, sp, 408
	sw	s3, 8(sp)
	addi	s4, sp, 280
	sw	s4, 12(sp)
	addi	s1, sp, 152
	sw	s1, 16(sp)
	addi	s0, sp, 24
	sw	s0, 20(sp)
	addi	a0, zero, 26
	addi	a1, sp, 536
	addi	a2, sp, 8
	jalr	s2
	mv	a0, zero
	addi	a1, sp, 544
	addi	a2, zero, 104
.LBB13_3:
	lw	a3, -8(a1)
	add	a4, s3, a0
	lw	a4, 0(a4)
	bne	a3, a4, .LBB13_10
	lw	a3, -4(a1)
	add	a4, s4, a0
	lw	a4, 0(a4)
	bne	a3, a4, .LBB13_10
	lw	a3, 0(a1)
	add	a4, s1, a0
	lw	a4, 0(a4)
	bne	a3, a4, .LBB13_10
	lw	a3, 4(a1)
	add	a4, s0, a0
	lw	a4, 0(a4)
	bne	a3, a4, .LBB13_10
	addi	a0, a0, 4
	addi	a1, a1, 16
	bne	a0, a2, .LBB13_3
	lw	a1, 512(sp)
	addi	a0, zero, 1
	beqz	a1, .LBB13_12
.LBB13_9:
	not	a0, a0
	andi	a0, a0, 1
	j	.LBB13_11
.LBB13_10:
	mv	a0, zero
.LBB13_11:
	lw	s4, 1048(sp)
	lw	s3, 1052(sp)
	lw	s2, 1056(sp)
	lw	s1, 1060(sp)
	lw	s0, 1064(sp)
	lw	ra, 1068(sp)
	addi	sp, sp, 1072
	ret
.LBB13_12:
	addi	a1, sp, 384
	addi	a2, sp, 256
	addi	a3, sp, 128
	addi	a4, sp, 516
	addi	a0, zero, 1
	addi	a5, zero, 26
	addi	s1, zero, 31
.LBB13_13:
	lw	s0, 0(a1)
	bnez	s0, .LBB13_9
	lw	s0, 0(a2)
	bnez	s0, .LBB13_9
	lw	s0, 0(a3)
	bnez	s0, .LBB13_9
	sltiu	a0, a5, 31
	beq	a5, s1, .LBB13_9
	lw	s0, 0(a4)
	addi	a1, a1, 4
	addi	a2, a2, 4
	addi	a3, a3, 4
	addi	a4, a4, 4
	addi	a5, a5, 1
	beqz	s0, .LBB13_13
	j	.LBB13_9
.Lfunc_end13:
	.size	vector_memcpy_segmented_harness_i32, .Lfunc_end13-vector_memcpy_segmented_harness_i32

	.globl	main
	.p2align	2
	.type	main,@function
main:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	sw	s0, 8(sp)
	lui	a0, %hi(vector_memcpy_8m8)
	addi	a0, a0, %lo(vector_memcpy_8m8)
	call	vector_memcpy_harness
	mv	s0, a0
	lui	a0, %hi(vector_memcpy_16m8)
	addi	a0, a0, %lo(vector_memcpy_16m8)
	call	vector_memcpy_harness
	slli	a0, a0, 1
	or	s0, a0, s0
	lui	a0, %hi(vector_memcpy_32m8)
	addi	a0, a0, %lo(vector_memcpy_32m8)
	call	vector_memcpy_harness
	slli	a0, a0, 2
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_32mf2)
	addi	a0, a0, %lo(vector_memcpy_32mf2)
	call	vector_memcpy_harness
	slli	a0, a0, 3
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_8strided)
	addi	a0, a0, %lo(vector_memcpy_8strided)
	call	vector_memcpy_harness
	slli	a0, a0, 4
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_16strided)
	addi	a0, a0, %lo(vector_memcpy_16strided)
	call	vector_memcpy_harness
	slli	a0, a0, 5
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_32strided)
	addi	a0, a0, %lo(vector_memcpy_32strided)
	call	vector_memcpy_harness
	slli	a0, a0, 6
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_indexed)
	addi	a0, a0, %lo(vector_memcpy_indexed)
	call	vector_memcpy_harness
	slli	a0, a0, 7
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_masked)
	addi	a0, a0, %lo(vector_memcpy_masked)
	call	vector_memcpy_masked_harness
	slli	a0, a0, 8
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_32m2_seg4load)
	addi	a0, a0, %lo(vector_memcpy_32m2_seg4load)
	call	vector_memcpy_segmented_harness_i32
	slli	a0, a0, 9
	or	a0, s0, a0
	lui	a1, 983040
	sw	a0, 0(a1)
	lw	s0, 8(sp)
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end14:
	.size	main, .Lfunc_end14-main

	.ident	"Ubuntu clang version 13.0.1-++20220120110924+75e33f71c2da-1~exp1~20220120231001.58"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym vector_memcpy_indexed
	.addrsig_sym vector_memcpy_masked
	.addrsig_sym vector_memcpy_8strided
	.addrsig_sym vector_memcpy_16strided
	.addrsig_sym vector_memcpy_32strided
	.addrsig_sym vector_memcpy_32mf2
	.addrsig_sym vector_memcpy_8m8
	.addrsig_sym vector_memcpy_16m8
	.addrsig_sym vector_memcpy_32m8
	.addrsig_sym vector_memcpy_32m2_seg4load
