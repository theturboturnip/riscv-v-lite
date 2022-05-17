	.file	"vector_memcpy_old.c"
	.option nopic
	.attribute arch, "rv64i2p0_m2p0_v1p0_zvamo1p0_zvlsseg1p0"
	.attribute unaligned_access, 0
	.attribute stack_align, 16
	.text
	.align	2
	.globl	vector_memcpy_indexed
	.type	vector_memcpy_indexed, @function
vector_memcpy_indexed:
	addi	sp,sp,-544
	sd	ra,536(sp)
	sd	s0,528(sp)
	sd	s1,520(sp)
	sd	s2,512(sp)
	mv	s0,a0
	mv	s2,a1
	mv	s1,a2
	li	a2,512
	li	a1,0
	mv	a0,sp
	call	memset
	li	a1,-1
	vsetvli	a1,a1,e32,m4
	mv	a2,a1
	beq	a1,zero,.L2
	mv	a3,sp
	li	a5,0
.L3:
	xori	a4,a5,1
	slliw	a4,a4,2
	sw	a4,0(a3)
	addi	a5,a5,1
	addi	a3,a3,4
	bne	a2,a5,.L3
.L2:
	vsetvli	a1,a1,e32,m4
	vle32.v	v8,0(sp)
	bne	s0,zero,.L7
.L1:
	ld	ra,536(sp)
	ld	s0,528(sp)
	ld	s1,520(sp)
	ld	s2,512(sp)
	addi	sp,sp,544
	jr	ra
.L11:
	vsetvli	a4,a5,e32,m4
	vloxei32.v	v4,(s2),v8
	vsetvli	a4,a5,e32,m4
	vsoxei32.v	v4,(s1),v8
.L6:
	slli	a4,a5,2
	add	s2,s2,a4
	add	s1,s1,a4
	sub	s0,s0,a5
	beq	s0,zero,.L1
.L7:
	vsetvli	a5,s0,e32,m4
	beq	a2,a5,.L11
	vsetvli	a4,a5,e32,m4
	vle32.v	v4,0(s2)
	vsetvli	a4,a5,e32,m4
	vse32.v	v4,0(s1)
	j	.L6
	.size	vector_memcpy_indexed, .-vector_memcpy_indexed
	.align	2
	.globl	vector_memcpy_masked
	.type	vector_memcpy_masked, @function
vector_memcpy_masked:
	addi	sp,sp,-544
	sd	ra,536(sp)
	sd	s0,528(sp)
	sd	s1,520(sp)
	sd	s2,512(sp)
	mv	s0,a0
	mv	s2,a1
	mv	s1,a2
	li	a2,512
	li	a1,0
	mv	a0,sp
	call	memset
	li	a2,-1
	vsetvli	a2,a2,e32,m4
	beq	a2,zero,.L13
	mv	a1,a2
	mv	a4,sp
	li	a5,0
.L14:
	andi	a3,a5,1
	sw	a3,0(a4)
	addi	a5,a5,1
	addi	a4,a4,4
	bne	a1,a5,.L14
.L13:
	vsetvli	a5,a2,e32,m4
	vle32.v	v4,0(sp)
	vsetvli	a5,a2,e32,m4
	vmseq.vi	v0,v4,1
	vsetvli	a2,a2,e32,m4
	vmv.v.i	v8,0
	beq	s0,zero,.L12
.L16:
	vsetvli	a5,s0,e32,m4
	vsetvli	a4,a5,e32,m4
	vmv4r.v	v4,v8
	vle32.v	v4,0(s2),v0.t
	vmv4r.v	v4,v4
	vsetvli	a4,a5,e32,m4
	vse32.v	v4,0(s1),v0.t
	slli	a4,a5,2
	add	s2,s2,a4
	add	s1,s1,a4
	sub	s0,s0,a5
	bne	s0,zero,.L16
.L12:
	ld	ra,536(sp)
	ld	s0,528(sp)
	ld	s1,520(sp)
	ld	s2,512(sp)
	addi	sp,sp,544
	jr	ra
	.size	vector_memcpy_masked, .-vector_memcpy_masked
	.align	2
	.globl	vector_memcpy_8strided
	.type	vector_memcpy_8strided, @function
vector_memcpy_8strided:
	beq	a0,zero,.L20
	li	t4,-1
	srli	t4,t4,2
	li	a6,4
	j	.L25
.L28:
	mv	a4,a1
	mv	a3,a2
	addi	a7,a1,16
.L23:
	vsetvli	t1,a5,e8,m1
	vlse8.v	v1,(a4),a6
	vsetvli	t1,a5,e8,m1
	vsse8.v	v1,(a3),a6
	addi	a4,a4,1
	addi	a3,a3,1
	bne	a4,a7,.L23
	slli	a5,a5,2
	add	a1,a1,a5
	add	a2,a2,a5
	sub	a0,a0,t3
.L24:
	beq	a0,zero,.L20
.L25:
	slli	a5,a0,2
	vsetvli	a5,a5,e8,m1
	and	t3,a5,t4
	bltu	t3,a0,.L28
	vsetvli	a4,a5,e8,m1
	vle8.v	v1,0(a1)
	vsetvli	a4,a5,e8,m1
	vse8.v	v1,0(a2)
	andi	a4,a5,-4
	add	a1,a1,a4
	add	a2,a2,a4
	srli	a5,a5,2
	sub	a0,a0,a5
	j	.L24
.L20:
	ret
	.size	vector_memcpy_8strided, .-vector_memcpy_8strided
	.align	2
	.globl	vector_memcpy_16strided
	.type	vector_memcpy_16strided, @function
vector_memcpy_16strided:
	li	t5,0
	li	t4,1
	li	a6,8
	bne	a0,zero,.L30
	ret
.L35:
	slli	a5,a0,1
	vsetvli	a5,a5,e16,m1
	mv	t5,a5
	slli	a7,a5,2
	bltu	a7,a0,.L39
	vsetvli	a4,a5,e16,m1
	vle16.v	v1,0(a1)
	vsetvli	a4,a5,e16,m1
	vse16.v	v1,0(a2)
	srli	a5,a5,1
	slli	a4,a5,2
	add	a1,a1,a4
	add	a2,a2,a4
	sub	a0,a0,a5
.L34:
	beq	a0,zero,.L29
.L30:
	bne	a0,t4,.L35
	bgtu	t5,t4,.L35
.L29:
	ret
.L39:
	mv	a4,a1
	mv	a3,a2
	addi	t1,a1,16
.L33:
	vsetvli	t3,a5,e16,m1
	vlse16.v	v1,(a4),a6
	vsetvli	t3,a5,e16,m1
	vsse16.v	v1,(a3),a6
	addi	a4,a4,2
	addi	a3,a3,2
	bne	t1,a4,.L33
	srli	a5,a7,1
	slli	a7,a7,1
	add	a1,a1,a7
	add	a2,a2,a7
	sub	a0,a0,a5
	j	.L34
	.size	vector_memcpy_16strided, .-vector_memcpy_16strided
	.align	2
	.globl	vector_memcpy_32strided
	.type	vector_memcpy_32strided, @function
vector_memcpy_32strided:
	beq	a0,zero,.L40
	li	a6,16
	j	.L45
.L47:
	li	a4,0
.L43:
	vsetvli	a7,a5,e32,m1
	add	a7,a1,a4
	vlse32.v	v1,(a7),a6
	vsetvli	a7,a5,e32,m1
	add	a7,a2,a4
	vsse32.v	v1,(a7),a6
	addi	a4,a4,4
	bne	a4,a6,.L43
	slli	a5,a5,4
	add	a1,a1,a5
	add	a2,a2,a5
	sub	a0,a0,a3
.L45:
	vsetvli	a5,a0,e32,m1
	slli	a3,a5,2
	bltu	a3,a0,.L47
	vsetvli	a4,a5,e32,m1
	vle32.v	v1,0(a1)
	vsetvli	a4,a5,e32,m1
	vse32.v	v1,0(a2)
	add	a1,a1,a3
	add	a2,a2,a3
	sub	a0,a0,a5
	bne	a0,zero,.L45
.L40:
	ret
	.size	vector_memcpy_32strided, .-vector_memcpy_32strided
	.align	2
	.globl	vector_memcpy_32m1_wholereg
	.type	vector_memcpy_32m1_wholereg, @function
vector_memcpy_32m1_wholereg:
	li	a3,-1
	vsetvli	a3,a3,e32,m1
	bne	a0,zero,.L52
.L48:
	ret
.L54:
 #APP
# 415 "/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/vector_memcpy_old/vector_memcpy_old.c" 1
	vl1r.v v1, (a1)
# 0 "" 2
 #NO_APP
	vmv1r.v	v1,v1
 #APP
# 420 "/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/vector_memcpy_old/vector_memcpy_old.c" 1
	vs1r.v v1, (a2)
# 0 "" 2
 #NO_APP
.L51:
	slli	a4,a5,2
	add	a1,a1,a4
	add	a2,a2,a4
	sub	a0,a0,a5
	beq	a0,zero,.L48
.L52:
	vsetvli	a5,a0,e32,m1
	beq	a3,a5,.L54
	vsetvli	a4,a5,e32,m1
	vle32.v	v1,0(a1)
	vsetvli	a4,a5,e32,m1
	vse32.v	v1,0(a2)
	j	.L51
	.size	vector_memcpy_32m1_wholereg, .-vector_memcpy_32m1_wholereg
	.align	2
	.globl	vector_memcpy_8m8
	.type	vector_memcpy_8m8, @function
vector_memcpy_8m8:
	slli	a0,a0,2
	beq	a0,zero,.L55
.L57:
	vsetvli	a5,a0,e8,m8
	vsetvli	a4,a5,e8,m8
	vle8.v	v8,0(a1)
	vsetvli	a4,a5,e8,m8
	vse8.v	v8,0(a2)
	add	a1,a1,a5
	add	a2,a2,a5
	sub	a0,a0,a5
	bne	a0,zero,.L57
.L55:
	ret
	.size	vector_memcpy_8m8, .-vector_memcpy_8m8
	.align	2
	.globl	vector_memcpy_16m8
	.type	vector_memcpy_16m8, @function
vector_memcpy_16m8:
	li	a4,0
	li	a6,1
	bne	a0,zero,.L60
	ret
.L62:
	slli	a5,a0,1
	vsetvli	a5,a5,e16,m8
	mv	a4,a5
	vsetvli	a3,a5,e16,m8
	vle16.v	v8,0(a1)
	vsetvli	a3,a5,e16,m8
	vse16.v	v8,0(a2)
	srli	a5,a5,1
	slli	a3,a5,2
	add	a1,a1,a3
	add	a2,a2,a3
	sub	a0,a0,a5
	beq	a0,zero,.L59
.L60:
	bne	a0,a6,.L62
	bgtu	a4,a6,.L62
.L59:
	ret
	.size	vector_memcpy_16m8, .-vector_memcpy_16m8
	.align	2
	.globl	vector_memcpy_32m8
	.type	vector_memcpy_32m8, @function
vector_memcpy_32m8:
	beq	a0,zero,.L65
.L67:
	vsetvli	a5,a0,e32,m8
	vsetvli	a4,a5,e32,m8
	vle32.v	v8,0(a1)
	vsetvli	a4,a5,e32,m8
	vse32.v	v8,0(a2)
	slli	a4,a5,2
	add	a1,a1,a4
	add	a2,a2,a4
	sub	a0,a0,a5
	bne	a0,zero,.L67
.L65:
	ret
	.size	vector_memcpy_32m8, .-vector_memcpy_32m8
	.align	2
	.globl	memset
	.type	memset, @function
memset:
	andi	a1,a1,0xff
	beq	a2,zero,.L70
	mv	a5,a0
	add	a2,a2,a0
.L71:
	sb	a1,0(a5)
	addi	a5,a5,1
	bne	a5,a2,.L71
.L70:
	ret
	.size	memset, .-memset
	.align	2
	.globl	vector_memcpy_harness
	.type	vector_memcpy_harness, @function
vector_memcpy_harness:
	addi	sp,sp,-1056
	sd	ra,1048(sp)
	sd	s0,1040(sp)
	sd	s1,1032(sp)
	sd	s2,1024(sp)
	mv	s2,a0
	li	a2,512
	li	a1,0
	addi	a0,sp,512
	call	memset
	li	a2,512
	li	a1,0
	mv	a0,sp
	call	memset
	addi	s0,sp,512
	mv	a4,s0
	li	a5,0
	li	a3,128
.L74:
	sw	a5,0(a4)
	addiw	a5,a5,1
	addi	a4,a4,4
	bne	a5,a3,.L74
	mv	s1,sp
	mv	a2,s1
	addi	a1,sp,512
	li	a0,103
	jalr	s2
	mv	a1,s1
	addi	a3,s0,412
	mv	a2,s1
.L76:
	lw	a4,0(s0)
	lw	a5,0(a2)
	bne	a4,a5,.L78
	addi	s0,s0,4
	addi	a2,a2,4
	bne	s0,a3,.L76
	addi	a5,sp,412
	addi	a3,a1,512
.L77:
	lw	a4,0(a5)
	bne	a4,zero,.L79
	addi	a5,a5,4
	bne	a5,a3,.L77
	li	a0,1
	j	.L75
.L78:
	li	a0,0
.L75:
	ld	ra,1048(sp)
	ld	s0,1040(sp)
	ld	s1,1032(sp)
	ld	s2,1024(sp)
	addi	sp,sp,1056
	jr	ra
.L79:
	li	a0,0
	j	.L75
	.size	vector_memcpy_harness, .-vector_memcpy_harness
	.align	2
	.globl	vector_memcpy_masked_harness
	.type	vector_memcpy_masked_harness, @function
vector_memcpy_masked_harness:
	addi	sp,sp,-1056
	sd	ra,1048(sp)
	sd	s0,1040(sp)
	sd	s1,1032(sp)
	sd	s2,1024(sp)
	mv	s2,a0
	li	a2,512
	li	a1,0
	addi	a0,sp,512
	call	memset
	li	a2,512
	li	a1,0
	mv	a0,sp
	call	memset
	li	a2,512
	li	a1,255
	mv	a0,sp
	call	memset
	addi	s1,sp,512
	mv	a4,s1
	li	a5,0
	li	a3,128
.L85:
	sw	a5,0(a4)
	addiw	a5,a5,1
	addi	a4,a4,4
	bne	a5,a3,.L85
	mv	s0,sp
	mv	a2,s0
	addi	a1,sp,512
	li	a0,103
	jalr	s2
	mv	a7,s0
	mv	a2,s0
	li	a5,0
	li	a6,-1
	li	a4,103
	j	.L89
.L86:
	lw	a3,0(a2)
	bne	a3,a6,.L87
.L88:
	addiw	a5,a5,1
	addi	a2,a2,4
	addi	s1,s1,4
	beq	a5,a4,.L97
.L89:
	andi	a0,a5,1
	beq	a0,zero,.L86
	lw	a1,0(s1)
	lw	a3,0(a2)
	beq	a1,a3,.L88
	li	a0,0
.L87:
	ld	ra,1048(sp)
	ld	s0,1040(sp)
	ld	s1,1032(sp)
	ld	s2,1024(sp)
	addi	sp,sp,1056
	jr	ra
.L97:
	addi	a5,sp,412
	addi	a3,a7,512
	li	a2,-1
.L90:
	lw	a4,0(a5)
	bne	a4,a2,.L92
	addi	a5,a5,4
	bne	a5,a3,.L90
	li	a0,1
	j	.L87
.L92:
	li	a0,0
	j	.L87
	.size	vector_memcpy_masked_harness, .-vector_memcpy_masked_harness
	.align	2
	.globl	main
	.type	main, @function
main:
	addi	sp,sp,-32
	sd	ra,24(sp)
	sd	s0,16(sp)
	sd	s1,8(sp)
	lui	a0,%hi(vector_memcpy_8m8)
	addi	a0,a0,%lo(vector_memcpy_8m8)
	call	vector_memcpy_harness
	mv	s0,a0
	lui	a0,%hi(vector_memcpy_16m8)
	addi	a0,a0,%lo(vector_memcpy_16m8)
	call	vector_memcpy_harness
	mv	s1,a0
	lui	a0,%hi(vector_memcpy_32m8)
	addi	a0,a0,%lo(vector_memcpy_32m8)
	call	vector_memcpy_harness
	slliw	s1,s1,1
	slliw	a0,a0,2
	or	s1,s1,a0
	or	s0,s0,s1
	sext.w	s0,s0
	lui	a0,%hi(vector_memcpy_8strided)
	addi	a0,a0,%lo(vector_memcpy_8strided)
	call	vector_memcpy_harness
	slliw	a0,a0,4
	or	s0,s0,a0
	sext.w	s0,s0
	lui	a0,%hi(vector_memcpy_16strided)
	addi	a0,a0,%lo(vector_memcpy_16strided)
	call	vector_memcpy_harness
	slliw	a0,a0,5
	or	s0,s0,a0
	sext.w	s0,s0
	lui	a0,%hi(vector_memcpy_32strided)
	addi	a0,a0,%lo(vector_memcpy_32strided)
	call	vector_memcpy_harness
	slliw	a0,a0,6
	or	s0,s0,a0
	sext.w	s0,s0
	lui	a0,%hi(vector_memcpy_indexed)
	addi	a0,a0,%lo(vector_memcpy_indexed)
	call	vector_memcpy_harness
	slliw	a0,a0,7
	or	s0,s0,a0
	sext.w	s0,s0
	lui	a0,%hi(vector_memcpy_masked)
	addi	a0,a0,%lo(vector_memcpy_masked)
	call	vector_memcpy_masked_harness
	slliw	a0,a0,8
	or	s0,s0,a0
	sext.w	s0,s0
	lui	a0,%hi(vector_memcpy_32m1_wholereg)
	addi	a0,a0,%lo(vector_memcpy_32m1_wholereg)
	call	vector_memcpy_harness
	slliw	a0,a0,13
	or	a0,s0,a0
	sext.w	a0,a0
	lui	a4,%hi(outputAttempted)
	li	a5,8192
	addi	a5,a5,503
	sw	a5,%lo(outputAttempted)(a4)
	lui	a5,%hi(outputSucceeded)
	sw	a0,%lo(outputSucceeded)(a5)
	ld	ra,24(sp)
	ld	s0,16(sp)
	ld	s1,8(sp)
	addi	sp,sp,32
	jr	ra
	.size	main, .-main
	.ident	"GCC: (GNU) 10.1.0"
