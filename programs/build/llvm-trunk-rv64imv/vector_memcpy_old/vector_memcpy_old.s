	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p0_m2p0_f2p0_d2p0_v1p0_zvl128b1p0_zvl32b1p0_zvl64b1p0"
	.file	"vector_memcpy_old.c"
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
	addi	sp, sp, -560
	sd	ra, 552(sp)
	sd	s0, 544(sp)
	sd	s1, 536(sp)
	sd	s2, 528(sp)
	sd	s3, 520(sp)
	mv	s0, a2
	mv	s1, a1
	mv	s2, a0
	addi	a0, sp, 8
	li	a2, 512
	addi	s3, sp, 8
	li	a1, 0
	call	memset@plt
	vsetvli	a0, zero, e32, m4, ta, mu
	beqz	a0, .LBB1_3
	li	a1, 0
.LBB1_2:
	slli	a2, a1, 2
	xori	a2, a2, 4
	sw	a2, 0(s3)
	addi	a1, a1, 1
	addi	s3, s3, 4
	bne	a0, a1, .LBB1_2
.LBB1_3:
	addi	a1, sp, 8
	vle32.v	v8, (a1)
	bnez	s2, .LBB1_7
.LBB1_4:
	ld	ra, 552(sp)
	ld	s0, 544(sp)
	ld	s1, 536(sp)
	ld	s2, 528(sp)
	ld	s3, 520(sp)
	addi	sp, sp, 560
	ret
.LBB1_5:
	vle32.v	v12, (s1)
	vse32.v	v12, (s0)
.LBB1_6:
	slli	a2, a1, 2
	add	s1, s1, a2
	sub	s2, s2, a1
	add	s0, s0, a2
	beqz	s2, .LBB1_4
.LBB1_7:
	vsetvli	a1, s2, e32, m4, ta, mu
	bne	a1, a0, .LBB1_5
	vloxei32.v	v12, (s1), v8
	vsoxei32.v	v12, (s0), v8
	j	.LBB1_6
.Lfunc_end1:
	.size	vector_memcpy_indexed, .Lfunc_end1-vector_memcpy_indexed

	.globl	vector_memcpy_masked
	.p2align	2
	.type	vector_memcpy_masked,@function
vector_memcpy_masked:
	addi	sp, sp, -560
	sd	ra, 552(sp)
	sd	s0, 544(sp)
	sd	s1, 536(sp)
	sd	s2, 528(sp)
	sd	s3, 520(sp)
	mv	s0, a2
	mv	s1, a1
	mv	s2, a0
	addi	a0, sp, 8
	li	a2, 512
	addi	s3, sp, 8
	li	a1, 0
	call	memset@plt
	vsetvli	a0, zero, e32, m4, ta, mu
	beqz	a0, .LBB2_3
	li	a1, 0
.LBB2_2:
	andi	a2, a1, 1
	sw	a2, 0(s3)
	addi	a1, a1, 1
	addi	s3, s3, 4
	bne	a0, a1, .LBB2_2
.LBB2_3:
	addi	a0, sp, 8
	vle32.v	v8, (a0)
	beqz	s2, .LBB2_6
	vmseq.vi	v0, v8, 1
	vmv.v.i	v8, 0
.LBB2_5:
	vsetvli	a0, s2, e32, m4, tu, mu
	vmv4r.v	v12, v8
	vle32.v	v12, (s1), v0.t
	vse32.v	v12, (s0), v0.t
	slli	a1, a0, 2
	add	s1, s1, a1
	sub	s2, s2, a0
	add	s0, s0, a1
	bnez	s2, .LBB2_5
.LBB2_6:
	ld	ra, 552(sp)
	ld	s0, 544(sp)
	ld	s1, 536(sp)
	ld	s2, 528(sp)
	ld	s3, 520(sp)
	addi	sp, sp, 560
	ret
.Lfunc_end2:
	.size	vector_memcpy_masked, .Lfunc_end2-vector_memcpy_masked

	.globl	vector_memcpy_masked_bytemaskload
	.p2align	2
	.type	vector_memcpy_masked_bytemaskload,@function
vector_memcpy_masked_bytemaskload:
	addi	sp, sp, -704
	sd	ra, 696(sp)
	sd	s0, 688(sp)
	sd	s1, 680(sp)
	sd	s2, 672(sp)
	sd	s3, 664(sp)
	sd	s4, 656(sp)
	csrr	a3, vlenb
	sub	sp, sp, a3
	mv	s0, a2
	mv	s1, a1
	mv	s2, a0
	addi	a0, sp, 144
	li	a2, 512
	addi	s4, sp, 144
	li	a1, 0
	call	memset@plt
	vsetvli	s3, zero, e32, m2, ta, mu
	beqz	s3, .LBB3_3
	li	a0, 0
.LBB3_2:
	andi	a1, a0, 1
	sw	a1, 0(s4)
	addi	a0, a0, 1
	addi	s4, s4, 4
	bne	s3, a0, .LBB3_2
.LBB3_3:
	addi	a0, sp, 144
	vle32.v	v8, (a0)
	vmseq.vi	v10, v8, 1
	addi	a0, sp, 656
	vs1r.v	v10, (a0)
	addi	a0, sp, 16
	li	a2, 128
	li	a1, 0
	call	memset@plt
	vsetvli	zero, s3, e8, mf2, ta, mu
	addi	a0, sp, 656
	vl1r.v	v8, (a0)
	addi	a0, sp, 16
	vsm.v	v8, (a0)
	addi	a0, sp, 16
	vlm.v	v0, (a0)
	beqz	s2, .LBB3_6
	vsetvli	zero, zero, e32, m2, ta, mu
	vmv.v.i	v8, 0
.LBB3_5:
	vsetvli	a0, s2, e32, m2, tu, mu
	vmv2r.v	v10, v8
	vle32.v	v10, (s1), v0.t
	vse32.v	v10, (s0), v0.t
	slli	a1, a0, 2
	add	s1, s1, a1
	sub	s2, s2, a0
	add	s0, s0, a1
	bnez	s2, .LBB3_5
.LBB3_6:
	csrr	a0, vlenb
	add	sp, sp, a0
	ld	ra, 696(sp)
	ld	s0, 688(sp)
	ld	s1, 680(sp)
	ld	s2, 672(sp)
	ld	s3, 664(sp)
	ld	s4, 656(sp)
	addi	sp, sp, 704
	ret
.Lfunc_end3:
	.size	vector_memcpy_masked_bytemaskload, .Lfunc_end3-vector_memcpy_masked_bytemaskload

	.globl	vector_memcpy_8strided
	.p2align	2
	.type	vector_memcpy_8strided,@function
vector_memcpy_8strided:
	beqz	a0, .LBB4_7
	li	a3, 4
	li	a4, 16
	j	.LBB4_4
.LBB4_2:
	vle8.v	v8, (a1)
	vse8.v	v8, (a2)
	srli	a5, a5, 2
.LBB4_3:
	sub	a0, a0, a5
	slli	a5, a5, 2
	add	a1, a1, a5
	add	a2, a2, a5
	beqz	a0, .LBB4_7
.LBB4_4:
	slli	a5, a0, 2
	vsetvli	a5, a5, e8, m1, ta, mu
	bgeu	a5, a0, .LBB4_2
	li	a6, 0
.LBB4_6:
	add	a7, a1, a6
	vsetvli	zero, a5, e8, m1, ta, mu
	vlse8.v	v8, (a7), a3
	add	a7, a2, a6
	addi	a6, a6, 1
	vsse8.v	v8, (a7), a3
	bne	a6, a4, .LBB4_6
	j	.LBB4_3
.LBB4_7:
	ret
.Lfunc_end4:
	.size	vector_memcpy_8strided, .Lfunc_end4-vector_memcpy_8strided

	.globl	vector_memcpy_16strided
	.p2align	2
	.type	vector_memcpy_16strided,@function
vector_memcpy_16strided:
	beqz	a0, .LBB5_9
	li	a6, 0
	li	a3, 1
	li	a4, 8
	li	a5, 16
	j	.LBB5_4
.LBB5_2:
	vle16.v	v8, (a1)
	vse16.v	v8, (a2)
	srli	a7, a6, 1
.LBB5_3:
	sub	a0, a0, a7
	slli	a7, a7, 2
	add	a1, a1, a7
	add	a2, a2, a7
	beqz	a0, .LBB5_9
.LBB5_4:
	addi	a7, a0, -1
	snez	a7, a7
	sltu	a6, a3, a6
	or	a6, a7, a6
	beqz	a6, .LBB5_9
	slli	a6, a0, 1
	vsetvli	a6, a6, e16, m1, ta, mu
	slli	a7, a6, 2
	bgeu	a7, a0, .LBB5_2
	li	a7, 0
.LBB5_7:
	add	t0, a1, a7
	vsetvli	zero, a6, e16, m1, ta, mu
	vlse16.v	v8, (t0), a4
	add	t0, a2, a7
	addi	a7, a7, 2
	vsse16.v	v8, (t0), a4
	bne	a7, a5, .LBB5_7
	slli	a7, a6, 1
	j	.LBB5_3
.LBB5_9:
	ret
.Lfunc_end5:
	.size	vector_memcpy_16strided, .Lfunc_end5-vector_memcpy_16strided

	.globl	vector_memcpy_32strided
	.p2align	2
	.type	vector_memcpy_32strided,@function
vector_memcpy_32strided:
	beqz	a0, .LBB6_8
	li	a3, 16
	j	.LBB6_4
.LBB6_2:
	vle32.v	v8, (a1)
	vse32.v	v8, (a2)
.LBB6_3:
	sub	a0, a0, a4
	slli	a4, a4, 2
	add	a1, a1, a4
	add	a2, a2, a4
	beqz	a0, .LBB6_8
.LBB6_4:
	vsetvli	a4, a0, e32, m1, ta, mu
	slli	a5, a4, 2
	bgeu	a5, a0, .LBB6_2
	li	a6, 0
.LBB6_6:
	add	a7, a1, a6
	vsetvli	zero, a4, e32, m1, ta, mu
	vlse32.v	v8, (a7), a3
	add	a7, a2, a6
	addi	a6, a6, 4
	vsse32.v	v8, (a7), a3
	bne	a6, a3, .LBB6_6
	mv	a4, a5
	j	.LBB6_3
.LBB6_8:
	ret
.Lfunc_end6:
	.size	vector_memcpy_32strided, .Lfunc_end6-vector_memcpy_32strided

	.globl	vector_memcpy_32mf2
	.p2align	2
	.type	vector_memcpy_32mf2,@function
vector_memcpy_32mf2:
	beqz	a0, .LBB7_2
.LBB7_1:
	vsetvli	a3, a0, e32, mf2, ta, mu
	vle32.v	v8, (a1)
	vse32.v	v8, (a2)
	slli	a4, a3, 2
	add	a1, a1, a4
	sub	a0, a0, a3
	add	a2, a2, a4
	bnez	a0, .LBB7_1
.LBB7_2:
	ret
.Lfunc_end7:
	.size	vector_memcpy_32mf2, .Lfunc_end7-vector_memcpy_32mf2

	.globl	vector_memcpy_8m8
	.p2align	2
	.type	vector_memcpy_8m8,@function
vector_memcpy_8m8:
	slli	a0, a0, 2
	beqz	a0, .LBB8_2
.LBB8_1:
	vsetvli	a3, a0, e8, m8, ta, mu
	vle8.v	v8, (a1)
	vse8.v	v8, (a2)
	add	a1, a1, a3
	sub	a0, a0, a3
	add	a2, a2, a3
	bnez	a0, .LBB8_1
.LBB8_2:
	ret
.Lfunc_end8:
	.size	vector_memcpy_8m8, .Lfunc_end8-vector_memcpy_8m8

	.globl	vector_memcpy_16m8
	.p2align	2
	.type	vector_memcpy_16m8,@function
vector_memcpy_16m8:
	beqz	a0, .LBB9_4
	li	a4, 0
	li	a3, 1
.LBB9_2:
	addi	a5, a0, -1
	snez	a5, a5
	sltu	a4, a3, a4
	or	a4, a5, a4
	beqz	a4, .LBB9_4
	slli	a4, a0, 1
	vsetvli	a4, a4, e16, m8, ta, mu
	vle16.v	v8, (a1)
	vse16.v	v8, (a2)
	srli	a5, a4, 1
	slli	a6, a5, 2
	add	a1, a1, a6
	sub	a0, a0, a5
	add	a2, a2, a6
	bnez	a0, .LBB9_2
.LBB9_4:
	ret
.Lfunc_end9:
	.size	vector_memcpy_16m8, .Lfunc_end9-vector_memcpy_16m8

	.globl	vector_memcpy_32m8
	.p2align	2
	.type	vector_memcpy_32m8,@function
vector_memcpy_32m8:
	beqz	a0, .LBB10_2
.LBB10_1:
	vsetvli	a3, a0, e32, m8, ta, mu
	vle32.v	v8, (a1)
	vse32.v	v8, (a2)
	slli	a4, a3, 2
	add	a1, a1, a4
	sub	a0, a0, a3
	add	a2, a2, a4
	bnez	a0, .LBB10_1
.LBB10_2:
	ret
.Lfunc_end10:
	.size	vector_memcpy_32m8, .Lfunc_end10-vector_memcpy_32m8

	.globl	vector_memcpy_32m1_wholereg
	.p2align	2
	.type	vector_memcpy_32m1_wholereg,@function
vector_memcpy_32m1_wholereg:
	vsetvli	a3, zero, e32, m1, ta, mu
	bnez	a0, .LBB11_4
.LBB11_1:
	ret
.LBB11_2:
	vle32.v	v8, (a1)
	vse32.v	v8, (a2)
.LBB11_3:
	slli	a5, a4, 2
	add	a1, a1, a5
	sub	a0, a0, a4
	add	a2, a2, a5
	beqz	a0, .LBB11_1
.LBB11_4:
	vsetvli	a4, a0, e32, m1, ta, mu
	bne	a4, a3, .LBB11_2
	#APP
	vl1r.v	v8, (a1)
	#NO_APP
	#APP
	vs1r.v	v8, (a2)
	#NO_APP
	j	.LBB11_3
.Lfunc_end11:
	.size	vector_memcpy_32m1_wholereg, .Lfunc_end11-vector_memcpy_32m1_wholereg

	.globl	vector_memcpy_32m8_faultonlyfirst
	.p2align	2
	.type	vector_memcpy_32m8_faultonlyfirst,@function
vector_memcpy_32m8_faultonlyfirst:
	bnez	a0, .LBB12_3
.LBB12_1:
	ret
.LBB12_2:
	xor	a4, a4, a3
	snez	a4, a4
	sub	a0, a0, a3
	seqz	a3, a0
	or	a3, a4, a3
	bnez	a3, .LBB12_1
.LBB12_3:
	vsetvli	a3, a0, e32, m8, ta, mu
	vle32ff.v	v8, (a1)
	csrr	a4, vl
	bne	a4, a3, .LBB12_2
	vsetvli	zero, a3, e32, m8, ta, mu
	vse32.v	v8, (a2)
	slli	a5, a3, 2
	add	a1, a1, a5
	add	a2, a2, a5
	j	.LBB12_2
.Lfunc_end12:
	.size	vector_memcpy_32m8_faultonlyfirst, .Lfunc_end12-vector_memcpy_32m8_faultonlyfirst

	.globl	vector_memcpy_32m2_seg4load
	.p2align	2
	.type	vector_memcpy_32m2_seg4load,@function
vector_memcpy_32m2_seg4load:
	beqz	a0, .LBB13_5
	li	a3, 32
.LBB13_2:
	vsetvli	a4, a0, e32, m2, ta, mu
	vlseg4e32.v	v8, (a1)
	ld	a5, 0(a2)
	vse32.v	v8, (a5)
	ld	a5, 8(a2)
	vse32.v	v10, (a5)
	ld	a5, 16(a2)
	vse32.v	v12, (a5)
	ld	a5, 24(a2)
	li	a6, 0
	vse32.v	v14, (a5)
	slli	a5, a4, 2
.LBB13_3:
	add	a7, a2, a6
	ld	t0, 0(a7)
	add	t0, t0, a5
	addi	a6, a6, 8
	sd	t0, 0(a7)
	bne	a6, a3, .LBB13_3
	slli	a5, a5, 2
	sub	a0, a0, a4
	add	a1, a1, a5
	bnez	a0, .LBB13_2
.LBB13_5:
	ret
.Lfunc_end13:
	.size	vector_memcpy_32m2_seg4load, .Lfunc_end13-vector_memcpy_32m2_seg4load

	.globl	vector_memcpy_harness
	.p2align	2
	.type	vector_memcpy_harness,@function
vector_memcpy_harness:
	addi	sp, sp, -1072
	sd	ra, 1064(sp)
	sd	s0, 1056(sp)
	sd	s1, 1048(sp)
	sd	s2, 1040(sp)
	mv	s0, a0
	addi	a0, sp, 528
	li	a2, 512
	addi	s1, sp, 528
	li	a1, 0
	call	memset@plt
	addi	a0, sp, 16
	li	a2, 512
	li	a1, 0
	call	memset@plt
	li	a0, 0
	li	a1, 128
.LBB14_1:
	sw	a0, 0(s1)
	addi	a0, a0, 1
	addi	s1, s1, 4
	bne	a0, a1, .LBB14_1
	addi	s1, sp, 528
	li	a0, 103
	addi	a1, sp, 528
	addi	a2, sp, 16
	addi	s2, sp, 16
	jalr	s0
	li	a0, 0
	li	a1, 412
.LBB14_3:
	add	a2, s1, a0
	lw	a2, 0(a2)
	add	a3, s2, a0
	lw	a3, 0(a3)
	bne	a2, a3, .LBB14_10
	addi	a0, a0, 4
	bne	a0, a1, .LBB14_3
	lw	a1, 428(sp)
	li	a0, 0
	bnez	a1, .LBB14_11
	addi	a0, sp, 432
	li	a3, 103
	li	a1, 127
.LBB14_7:
	mv	a2, a3
	beq	a3, a1, .LBB14_9
	lw	a4, 0(a0)
	addi	a0, a0, 4
	addi	a3, a2, 1
	beqz	a4, .LBB14_7
.LBB14_9:
	li	a0, 126
	sltu	a0, a0, a2
	j	.LBB14_11
.LBB14_10:
	li	a0, 0
.LBB14_11:
	ld	ra, 1064(sp)
	ld	s0, 1056(sp)
	ld	s1, 1048(sp)
	ld	s2, 1040(sp)
	addi	sp, sp, 1072
	ret
.Lfunc_end14:
	.size	vector_memcpy_harness, .Lfunc_end14-vector_memcpy_harness

	.globl	vector_memcpy_masked_harness
	.p2align	2
	.type	vector_memcpy_masked_harness,@function
vector_memcpy_masked_harness:
	addi	sp, sp, -1072
	sd	ra, 1064(sp)
	sd	s0, 1056(sp)
	sd	s1, 1048(sp)
	sd	s2, 1040(sp)
	sd	s3, 1032(sp)
	mv	s0, a0
	addi	a0, sp, 520
	li	a2, 512
	addi	s1, sp, 520
	li	a1, 0
	call	memset@plt
	addi	a0, sp, 8
	li	a2, 512
	li	a1, 0
	call	memset@plt
	addi	a0, sp, 8
	li	a1, 255
	li	a2, 512
	call	memset
	li	a0, 0
	li	a1, 128
.LBB15_1:
	sw	a0, 0(s1)
	addi	a0, a0, 1
	addi	s1, s1, 4
	bne	a0, a1, .LBB15_1
	li	s1, 103
	addi	s2, sp, 520
	li	a0, 103
	addi	a1, sp, 520
	addi	a2, sp, 8
	addi	s3, sp, 8
	jalr	s0
	li	a0, 0
	li	a1, -1
	j	.LBB15_5
.LBB15_3:
	lw	a2, 0(s2)
	lw	a3, 0(s3)
	bne	a2, a3, .LBB15_8
.LBB15_4:
	addi	a0, a0, 1
	addi	s3, s3, 4
	addi	s2, s2, 4
	beq	a0, s1, .LBB15_7
.LBB15_5:
	andi	a2, a0, 1
	bnez	a2, .LBB15_3
	lw	a2, 0(s3)
	beq	a2, a1, .LBB15_4
	j	.LBB15_8
.LBB15_7:
	lw	a1, 420(sp)
	li	a0, -1
	beq	a1, a0, .LBB15_10
.LBB15_8:
	li	a0, 0
.LBB15_9:
	ld	ra, 1064(sp)
	ld	s0, 1056(sp)
	ld	s1, 1048(sp)
	ld	s2, 1040(sp)
	ld	s3, 1032(sp)
	addi	sp, sp, 1072
	ret
.LBB15_10:
	addi	a1, sp, 424
	li	a4, 103
	li	a2, 127
.LBB15_11:
	mv	a3, a4
	beq	a4, a2, .LBB15_13
	lw	a5, 0(a1)
	addi	a1, a1, 4
	addi	a4, a3, 1
	beq	a5, a0, .LBB15_11
.LBB15_13:
	li	a0, 126
	sltu	a0, a0, a3
	j	.LBB15_9
.Lfunc_end15:
	.size	vector_memcpy_masked_harness, .Lfunc_end15-vector_memcpy_masked_harness

	.globl	vector_memcpy_segmented_harness_i32
	.p2align	2
	.type	vector_memcpy_segmented_harness_i32,@function
vector_memcpy_segmented_harness_i32:
	addi	sp, sp, -1120
	sd	ra, 1112(sp)
	sd	s0, 1104(sp)
	sd	s1, 1096(sp)
	sd	s2, 1088(sp)
	sd	s3, 1080(sp)
	sd	s4, 1072(sp)
	mv	s0, a0
	addi	a0, sp, 560
	li	a2, 512
	addi	s1, sp, 560
	li	a1, 0
	call	memset@plt
	addi	a0, sp, 432
	li	a2, 128
	li	s2, 128
	li	a1, 0
	call	memset@plt
	addi	a0, sp, 304
	li	a2, 128
	li	a1, 0
	call	memset@plt
	addi	a0, sp, 176
	li	a2, 128
	li	a1, 0
	call	memset@plt
	addi	a0, sp, 48
	li	a2, 128
	li	a1, 0
	call	memset@plt
	li	a0, 0
.LBB16_1:
	sw	a0, 0(s1)
	addi	a0, a0, 1
	addi	s1, s1, 4
	bne	a0, s2, .LBB16_1
	addi	s1, sp, 432
	sd	s1, 16(sp)
	addi	s2, sp, 304
	sd	s2, 24(sp)
	addi	s3, sp, 176
	sd	s3, 32(sp)
	addi	s4, sp, 48
	sd	s4, 40(sp)
	li	a0, 26
	addi	a1, sp, 560
	addi	a2, sp, 16
	jalr	s0
	li	a0, 0
	addi	a1, sp, 568
	li	a2, 104
.LBB16_3:
	lw	a3, -8(a1)
	add	a4, s1, a0
	lw	a4, 0(a4)
	bne	a3, a4, .LBB16_10
	lw	a3, -4(a1)
	add	a4, s2, a0
	lw	a4, 0(a4)
	bne	a3, a4, .LBB16_10
	lw	a3, 0(a1)
	add	a4, s3, a0
	lw	a4, 0(a4)
	bne	a3, a4, .LBB16_10
	lw	a3, 4(a1)
	add	a4, s4, a0
	lw	a4, 0(a4)
	bne	a3, a4, .LBB16_10
	addi	a0, a0, 4
	addi	a1, a1, 16
	bne	a0, a2, .LBB16_3
	lw	a1, 536(sp)
	li	a0, 1
	beqz	a1, .LBB16_12
.LBB16_9:
	not	a0, a0
	andi	a0, a0, 1
	j	.LBB16_11
.LBB16_10:
	li	a0, 0
.LBB16_11:
	ld	ra, 1112(sp)
	ld	s0, 1104(sp)
	ld	s1, 1096(sp)
	ld	s2, 1088(sp)
	ld	s3, 1080(sp)
	ld	s4, 1072(sp)
	addi	sp, sp, 1120
	ret
.LBB16_12:
	addi	a1, sp, 540
	addi	a2, sp, 408
	addi	a3, sp, 280
	addi	a4, sp, 152
	li	a5, 26
	li	a0, 1
	li	a6, 31
.LBB16_13:
	lw	a7, 0(a2)
	bnez	a7, .LBB16_9
	lw	a7, 0(a3)
	bnez	a7, .LBB16_9
	lw	a7, 0(a4)
	bnez	a7, .LBB16_9
	sltiu	a0, a5, 31
	beq	a5, a6, .LBB16_9
	lw	a7, 0(a1)
	addi	a1, a1, 4
	addi	a2, a2, 4
	addi	a3, a3, 4
	addi	a4, a4, 4
	addi	a5, a5, 1
	beqz	a7, .LBB16_13
	j	.LBB16_9
.Lfunc_end16:
	.size	vector_memcpy_segmented_harness_i32, .Lfunc_end16-vector_memcpy_segmented_harness_i32

	.globl	vector_unit_faultonlyfirst_test_under_fault
	.p2align	2
	.type	vector_unit_faultonlyfirst_test_under_fault,@function
vector_unit_faultonlyfirst_test_under_fault:
	vsetvli	a1, zero, e32, m1, ta, mu
	beqz	a1, .LBB17_3
	li	a0, 0
	slli	a2, a1, 2
	lui	a3, 37
	sub	a2, a3, a2
.LBB17_2:
	sw	a0, 0(a2)
	addi	a0, a0, 1
	addi	a2, a2, 4
	bne	a1, a0, .LBB17_2
.LBB17_3:
	seqz	a0, a1
	beqz	a1, .LBB17_9
	vsetvli	a2, zero, e32, m1, ta, mu
	lui	a3, 37
	addiw	a2, a3, -4
	vle32ff.v	v8, (a2)
	csrr	a4, vl
	li	a2, 1
	bne	a4, a2, .LBB17_9
	addiw	a0, a3, -8
.LBB17_6:
	mv	a3, a2
	beq	a1, a2, .LBB17_8
	vsetvli	a2, zero, e32, m1, ta, mu
	vle32ff.v	v8, (a0)
	addi	a2, a3, 1
	csrr	a4, vl
	addi	a0, a0, -4
	beq	a2, a4, .LBB17_6
.LBB17_8:
	sltu	a0, a3, a1
	xori	a0, a0, 1
.LBB17_9:
	ret
.Lfunc_end17:
	.size	vector_unit_faultonlyfirst_test_under_fault, .Lfunc_end17-vector_unit_faultonlyfirst_test_under_fault

	.globl	main
	.p2align	2
	.type	main,@function
main:
	addi	sp, sp, -16
	sd	ra, 8(sp)
	sd	s0, 0(sp)
	lui	a0, %hi(vector_memcpy_8m8)
	addi	a0, a0, %lo(vector_memcpy_8m8)
	call	vector_memcpy_harness
	mv	s0, a0
	lui	a0, %hi(vector_memcpy_16m8)
	addi	a0, a0, %lo(vector_memcpy_16m8)
	call	vector_memcpy_harness
	slliw	a0, a0, 1
	or	s0, a0, s0
	lui	a0, %hi(vector_memcpy_32m8)
	addi	a0, a0, %lo(vector_memcpy_32m8)
	call	vector_memcpy_harness
	slliw	a0, a0, 2
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_32mf2)
	addi	a0, a0, %lo(vector_memcpy_32mf2)
	call	vector_memcpy_harness
	slliw	a0, a0, 3
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_8strided)
	addi	a0, a0, %lo(vector_memcpy_8strided)
	call	vector_memcpy_harness
	slliw	a0, a0, 4
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_16strided)
	addi	a0, a0, %lo(vector_memcpy_16strided)
	call	vector_memcpy_harness
	slliw	a0, a0, 5
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_32strided)
	addi	a0, a0, %lo(vector_memcpy_32strided)
	call	vector_memcpy_harness
	slliw	a0, a0, 6
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_indexed)
	addi	a0, a0, %lo(vector_memcpy_indexed)
	call	vector_memcpy_harness
	slliw	a0, a0, 7
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_masked)
	addi	a0, a0, %lo(vector_memcpy_masked)
	call	vector_memcpy_masked_harness
	slliw	a0, a0, 8
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_32m2_seg4load)
	addi	a0, a0, %lo(vector_memcpy_32m2_seg4load)
	call	vector_memcpy_segmented_harness_i32
	slliw	a0, a0, 9
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_masked_bytemaskload)
	addi	a0, a0, %lo(vector_memcpy_masked_bytemaskload)
	call	vector_memcpy_masked_harness
	slliw	a0, a0, 10
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_32m8_faultonlyfirst)
	addi	a0, a0, %lo(vector_memcpy_32m8_faultonlyfirst)
	call	vector_memcpy_harness
	slliw	a0, a0, 11
	or	s0, s0, a0
	call	vector_unit_faultonlyfirst_test_under_fault
	slliw	a0, a0, 12
	or	s0, s0, a0
	lui	a0, %hi(vector_memcpy_32m1_wholereg)
	addi	a0, a0, %lo(vector_memcpy_32m1_wholereg)
	call	vector_memcpy_harness
	slliw	a0, a0, 13
	or	a0, s0, a0
	slli	a1, a0, 32
	srli	a1, a1, 32
	lui	a2, %hi(outputAttempted)
	lui	a3, 4
	addiw	a3, a3, -1
	sd	a3, %lo(outputAttempted)(a2)
	lui	a2, %hi(outputSucceeded)
	sd	a1, %lo(outputSucceeded)(a2)
	lui	a1, %hi(finished)
	li	a2, 1
	sext.w	a0, a0
	sb	a2, %lo(finished)(a1)
	ld	ra, 8(sp)
	ld	s0, 0(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end18:
	.size	main, .Lfunc_end18-main

	.ident	"clang version 15.0.0 (https://github.com/llvm/llvm-project.git 853e0aa424e40b80d0bda1dd8a3471a361048e4b)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym vector_memcpy_indexed
	.addrsig_sym vector_memcpy_masked
	.addrsig_sym vector_memcpy_masked_bytemaskload
	.addrsig_sym vector_memcpy_8strided
	.addrsig_sym vector_memcpy_16strided
	.addrsig_sym vector_memcpy_32strided
	.addrsig_sym vector_memcpy_32mf2
	.addrsig_sym vector_memcpy_8m8
	.addrsig_sym vector_memcpy_16m8
	.addrsig_sym vector_memcpy_32m8
	.addrsig_sym vector_memcpy_32m1_wholereg
	.addrsig_sym vector_memcpy_32m8_faultonlyfirst
	.addrsig_sym vector_memcpy_32m2_seg4load
	.addrsig_sym outputAttempted
	.addrsig_sym outputSucceeded
	.addrsig_sym finished
