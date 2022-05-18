	.file	"vector_memcpy_pointers.cpp"
	.option nopic
	.attribute arch, "rv64i2p0_m2p0_v1p0_zvamo1p0_zvlsseg1p0"
	.attribute unaligned_access, 0
	.attribute stack_align, 16
	.text
.Ltext0:
	.align	2
	.globl	memset
	.type	memset, @function
memset:
.LFB17413:
	.file 1 "/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/vector_memcpy_pointers/vector_memcpy_pointers.cpp"
	.loc 1 8 48
	.cfi_startproc
.LVL0:
	.loc 1 9 5
	.loc 1 9 19 is_stmt 0
	andi	a1,a1,0xff
.LVL1:
	.loc 1 10 5 is_stmt 1
	.loc 1 11 5
.LBB17:
	.loc 1 11 23
	beq	a2,zero,.L2
	mv	a5,a0
	add	a2,a2,a0
.LVL2:
.L3:
	.loc 1 12 9 discriminator 2
	.loc 1 12 24 is_stmt 0 discriminator 2
	sb	a1,0(a5)
	.loc 1 11 5 is_stmt 1 discriminator 2
.LVL3:
	.loc 1 11 23 discriminator 2
	addi	a5,a5,1
.LVL4:
	bne	a5,a2,.L3
.L2:
.LBE17:
	.loc 1 15 5
	.loc 1 16 1 is_stmt 0
	ret
	.cfi_endproc
.LFE17413:
	.size	memset, .-memset
	.align	2
	.globl	memcpy
	.type	memcpy, @function
memcpy:
.LFB17414:
	.loc 1 19 57 is_stmt 1
	.cfi_startproc
.LVL5:
	.loc 1 20 5
	.loc 1 21 5
	.loc 1 22 5
	.loc 1 22 17
	beq	a2,zero,.L6
	add	a2,a0,a2
.LVL6:
	.loc 1 20 20 is_stmt 0
	mv	a5,a0
.LVL7:
.L7:
	.loc 1 23 9 is_stmt 1
	.loc 1 23 20 is_stmt 0
	lbu	a4,0(a1)
	.loc 1 23 18
	sb	a4,0(a5)
	.loc 1 24 9 is_stmt 1
	.loc 1 24 16 is_stmt 0
	addi	a5,a5,1
.LVL8:
	.loc 1 25 9 is_stmt 1
	.loc 1 25 15 is_stmt 0
	addi	a1,a1,1
.LVL9:
	.loc 1 26 9 is_stmt 1
	.loc 1 22 5
	.loc 1 22 17
	bne	a2,a5,.L7
.LVL10:
.L6:
	.loc 1 28 5
	.loc 1 29 1 is_stmt 0
	ret
	.cfi_endproc
.LFE17414:
	.size	memcpy, .-memcpy
	.align	2
	.globl	_Z13vector_memcpyPhPKhm
	.type	_Z13vector_memcpyPhPKhm, @function
_Z13vector_memcpyPhPKhm:
.LFB17415:
	.loc 1 156 72 is_stmt 1
	.cfi_startproc
.LVL11:
	.loc 1 175 5
	.loc 1 175 22
	beq	a2,zero,.L9
.L11:
.LBB18:
	.loc 1 176 9
.LVL12:
.LBB19:
.LBB20:
	.file 2 "/scratch/gccs/outputs/rvv-intrinsic/lib/gcc/riscv64-unknown-elf/10.1.0/include/riscv_vector.h"
	.loc 2 137 1
	.loc 2 137 1
	.loc 2 137 1
	vsetvli	a5,a2,e8,m8
.LVL13:
	.loc 2 137 1
.LBE20:
.LBE19:
	.loc 1 178 9
	.loc 1 183 9
.LBB21:
.LBB22:
	.loc 2 509 1
.LBB23:
.LBB24:
	.loc 2 137 1
	.loc 2 137 1
	.loc 2 137 1
	vsetvli	a4,a5,e8,m8
	.loc 2 137 1
.LVL14:
.LBE24:
.LBE23:
	.loc 2 509 1
	.loc 2 509 1
	vle8.v	v8,0(a1)
.LVL15:
.LBE22:
.LBE21:
	.loc 1 184 9
.LBB25:
.LBB26:
	.loc 2 510 1
.LBB27:
.LBB28:
	.loc 2 137 1
	.loc 2 137 1
	.loc 2 137 1
	vsetvli	a4,a5,e8,m8
	.loc 2 137 1
.LVL16:
.LBE28:
.LBE27:
	.loc 2 510 1
	.loc 2 510 1
	vse8.v	v8,0(a0)
.LVL17:
.LBE26:
.LBE25:
	.loc 1 187 9
	.loc 1 187 13 is_stmt 0
	add	a1,a1,a5
.LVL18:
	.loc 1 188 9 is_stmt 1
	.loc 1 188 13 is_stmt 0
	add	a0,a0,a5
.LVL19:
	.loc 1 189 9 is_stmt 1
	.loc 1 189 19 is_stmt 0
	sub	a2,a2,a5
.LVL20:
.LBE18:
	.loc 1 175 5 is_stmt 1
	.loc 1 175 22
	bne	a2,zero,.L11
.LVL21:
.L9:
	.loc 1 191 1 is_stmt 0
	ret
	.cfi_endproc
.LFE17415:
	.size	_Z13vector_memcpyPhPKhm, .-_Z13vector_memcpyPhPKhm
	.align	2
	.globl	_Z13run_base_testv
	.type	_Z13run_base_testv, @function
_Z13run_base_testv:
.LFB17416:
	.loc 1 239 25 is_stmt 1
	.cfi_startproc
	addi	sp,sp,-608
	.cfi_def_cfa_offset 608
	sd	ra,600(sp)
	sd	s0,592(sp)
	sd	s1,584(sp)
	li	t1,-8192
	add	sp,sp,t1
	.cfi_def_cfa_offset 8800
	.cfi_offset 1, -8
	.cfi_offset 8, -16
	.cfi_offset 9, -24
	.loc 1 243 5
	.loc 1 243 10 is_stmt 0
	lui	a3,%hi(.LANCHOR0)
	addi	a3,a3,%lo(.LANCHOR0)
	ld	t1,0(a3)
	ld	a7,8(a3)
	ld	a6,16(a3)
	ld	a0,24(a3)
	ld	a1,32(a3)
	ld	a2,40(a3)
	ld	a4,48(a3)
	ld	a5,56(a3)
	li	t3,8192
	addi	t4,t3,512
	add	t4,t4,sp
	sd	t1,0(t4)
	mv	t1,t3
	addi	t3,t3,520
	add	t3,t3,sp
	sd	a7,0(t3)
	mv	a7,t1
	addi	t1,t1,528
	add	t1,t1,sp
	sd	a6,0(t1)
	mv	a6,a7
	addi	a7,a7,536
	add	a7,a7,sp
	sd	a0,0(a7)
	mv	a0,a6
	addi	a6,a6,544
	add	a6,a6,sp
	sd	a1,0(a6)
	mv	a1,a0
	addi	a0,a0,552
	add	a0,a0,sp
	sd	a2,0(a0)
	mv	a2,a1
	addi	a1,a1,560
	add	a1,a1,sp
	sd	a4,0(a1)
	mv	a4,a2
	addi	a2,a2,568
	add	a2,a2,sp
	sd	a5,0(a2)
	.loc 1 252 5 is_stmt 1
	.loc 1 252 9 is_stmt 0
	addi	a5,a3,64
	add	a4,sp,a4
	addi	a3,a3,576
.L14:
	ld	a6,0(a5)
	ld	a0,8(a5)
	ld	a1,16(a5)
	ld	a2,24(a5)
	sd	a6,0(a4)
	sd	a0,8(a4)
	sd	a1,16(a4)
	sd	a2,24(a4)
	addi	a5,a5,32
	addi	a4,a4,32
	bne	a5,a3,.L14
	.loc 1 271 5 is_stmt 1
	.loc 1 272 5
.LVL22:
.LBB29:
	.loc 1 272 26
	li	a5,8192
	add	s0,sp,a5
	li	a4,-4096
	addi	a4,a4,-576
	addi	a3,a5,576
	add	a3,a3,sp
	add	a4,a3,a4
	mv	a1,s0
.LBE29:
	.loc 1 252 9 is_stmt 0
	mv	a3,s0
.LVL23:
.L15:
.LBB31:
.LBB30:
	.loc 1 273 9 is_stmt 1 discriminator 2
	.loc 1 273 13 is_stmt 0 discriminator 2
	lw	a5,0(a3)
.LVL24:
	.loc 1 274 9 is_stmt 1 discriminator 2
	.loc 1 275 49 is_stmt 0 discriminator 2
	slli	a5,a5,4
.LVL25:
	li	a0,8192
	addi	a2,a0,576
	add	a2,a2,sp
	add	a2,a2,a5
	.loc 1 274 25 discriminator 2
	ld	a2,-64(a2)
	sd	a2,0(a4)
	.loc 1 276 57 discriminator 2
	addi	a2,a0,512
	add	a2,a2,sp
	add	a5,a2,a5
	.loc 1 274 25 discriminator 2
	sd	a5,16(a4)
.LBE30:
	.loc 1 272 5 is_stmt 1 discriminator 2
	.loc 1 272 26 discriminator 2
	addi	a3,a3,4
.LVL26:
	addi	a4,a4,32
	bne	a4,a1,.L15
.LBE31:
	.loc 1 280 5
	.loc 1 280 13 is_stmt 0
	li	s1,-8192
	li	a2,4096
	li	a1,0
	mv	a0,sp
	call	memset
.LVL27:
	.loc 1 282 5 is_stmt 1
	.loc 1 283 5
	.loc 1 286 5
	.loc 1 286 18 is_stmt 0
	addi	s1,s1,-576
	li	a4,8192
	addi	a5,a4,576
	add	a5,a5,sp
	add	s1,a5,s1
	li	a2,4096
	li	a1,-4096
	addi	a1,a1,-576
	addi	a5,a4,576
	add	a5,a5,sp
	add	a1,a5,a1
	mv	a0,s1
	call	_Z13vector_memcpyPhPKhm
.LVL28:
.LBB32:
	.loc 1 289 26 is_stmt 1
	mv	a4,s1
	li	a1,4096
	add	a1,s1,a1
.LVL29:
.L17:
	.loc 1 292 9
	.loc 1 292 27 is_stmt 0
	ld	a3,16(a4)
	.loc 1 292 9
	ld	a2,0(a3)
	ld	a5,0(a4)
	bne	a2,a5,.L18
	.loc 1 295 9 is_stmt 1
	.loc 1 295 71 is_stmt 0
	lw	a5,0(s0)
	slli	a5,a5,4
	li	a2,8192
	addi	a2,a2,512
	add	a2,a2,sp
	add	a5,a2,a5
	.loc 1 295 9
	bne	a3,a5,.L19
	.loc 1 289 5 is_stmt 1 discriminator 2
	.loc 1 289 26 discriminator 2
	addi	a4,a4,32
	addi	s0,s0,4
	bne	a4,a1,.L17
.LBE32:
	.loc 1 299 12 is_stmt 0
	li	a0,1
.LBB33:
	j	.L16
.L18:
	.loc 1 293 20
	li	a0,0
.L16:
.LBE33:
	.loc 1 300 1
	li	t1,8192
	add	sp,sp,t1
	.cfi_remember_state
	.cfi_def_cfa_offset 608
	ld	ra,600(sp)
	.cfi_restore 1
	ld	s0,592(sp)
	.cfi_restore 8
	ld	s1,584(sp)
	.cfi_restore 9
	addi	sp,sp,608
	.cfi_def_cfa_offset 0
	jr	ra
.L19:
	.cfi_restore_state
.LBB34:
	.loc 1 296 20
	li	a0,0
	j	.L16
.LBE34:
	.cfi_endproc
.LFE17416:
	.size	_Z13run_base_testv, .-_Z13run_base_testv
	.align	2
	.globl	main
	.type	main, @function
main:
.LFB17417:
	.loc 1 374 1 is_stmt 1
	.cfi_startproc
	addi	sp,sp,-16
	.cfi_def_cfa_offset 16
	sd	ra,8(sp)
	.cfi_offset 1, -8
	.loc 1 375 5
.LVL30:
	.loc 1 376 5
	.loc 1 378 5
	.loc 1 379 5
	.loc 1 379 28 is_stmt 0
	call	_Z13run_base_testv
.LVL31:
	.loc 1 386 5 is_stmt 1
	.loc 1 386 25 is_stmt 0
	li	a4,1
	lui	a3,%hi(outputAttempted)
	sd	a4,%lo(outputAttempted)(a3)
	.loc 1 387 5 is_stmt 1
	.loc 1 387 25 is_stmt 0
	lui	a3,%hi(outputSucceeded)
	sd	a0,%lo(outputSucceeded)(a3)
	.loc 1 388 5 is_stmt 1
	.loc 1 388 14 is_stmt 0
	lui	a5,%hi(finished)
	sb	a4,%lo(finished)(a5)
	.loc 1 389 5 is_stmt 1
	.loc 1 390 1 is_stmt 0
	ld	ra,8(sp)
	.cfi_restore 1
	addi	sp,sp,16
	.cfi_def_cfa_offset 0
	jr	ra
	.cfi_endproc
.LFE17417:
	.size	main, .-main
	.section	.rodata
	.align	4
	.set	.LANCHOR0,. + 0
.LC0:
	.dword	8389908080790640474
	.zero	8
	.dword	-8293413467698580316
	.zero	8
	.dword	7433078584290569438
	.zero	8
	.dword	-7710342347414968681
	.zero	8
.LC1:
	.word	1
	.word	1
	.word	1
	.word	1
	.word	2
	.word	2
	.word	3
	.word	0
	.word	1
	.word	0
	.word	3
	.word	0
	.word	0
	.word	3
	.word	3
	.word	0
	.word	1
	.word	0
	.word	0
	.word	2
	.word	2
	.word	0
	.word	2
	.word	1
	.word	0
	.word	0
	.word	0
	.word	3
	.word	2
	.word	0
	.word	1
	.word	1
	.word	3
	.word	2
	.word	3
	.word	0
	.word	2
	.word	2
	.word	0
	.word	0
	.word	0
	.word	1
	.word	1
	.word	3
	.word	0
	.word	0
	.word	1
	.word	3
	.word	1
	.word	2
	.word	3
	.word	2
	.word	2
	.word	0
	.word	2
	.word	1
	.word	0
	.word	3
	.word	1
	.word	1
	.word	3
	.word	3
	.word	2
	.word	2
	.word	0
	.word	1
	.word	3
	.word	2
	.word	2
	.word	1
	.word	1
	.word	3
	.word	2
	.word	2
	.word	0
	.word	1
	.word	1
	.word	3
	.word	0
	.word	1
	.word	0
	.word	0
	.word	3
	.word	2
	.word	2
	.word	3
	.word	3
	.word	1
	.word	1
	.word	1
	.word	1
	.word	2
	.word	1
	.word	1
	.word	2
	.word	1
	.word	2
	.word	2
	.word	1
	.word	1
	.word	3
	.word	1
	.word	1
	.word	3
	.word	0
	.word	2
	.word	3
	.word	1
	.word	1
	.word	3
	.word	2
	.word	3
	.word	2
	.word	1
	.word	2
	.word	0
	.word	2
	.word	2
	.word	2
	.word	3
	.word	0
	.word	3
	.word	1
	.word	0
	.word	3
	.word	2
	.word	1
	.word	0
	.text
.Letext0:
	.file 3 "/scratch/gccs/outputs/rvv-intrinsic/lib/gcc/riscv64-unknown-elf/10.1.0/include/stddef.h"
	.file 4 "/scratch/gccs/outputs/rvv-intrinsic/lib/gcc/riscv64-unknown-elf/10.1.0/include/stdint-gcc.h"
	.section	.debug_info,"",@progbits
.Ldebug_info0:
	.4byte	0x636
	.2byte	0x4
	.4byte	.Ldebug_abbrev0
	.byte	0x8
	.byte	0x1
	.4byte	.LASF46
	.byte	0x4
	.4byte	.LASF47
	.4byte	.LASF48
	.8byte	.Ltext0
	.8byte	.Letext0-.Ltext0
	.4byte	.Ldebug_line0
	.byte	0x2
	.byte	0x8
	.byte	0x5
	.4byte	.LASF0
	.byte	0x3
	.4byte	.LASF4
	.byte	0x3
	.byte	0xd1
	.byte	0x17
	.4byte	0x40
	.byte	0x2
	.byte	0x8
	.byte	0x7
	.4byte	.LASF1
	.byte	0x2
	.byte	0x8
	.byte	0x5
	.4byte	.LASF2
	.byte	0x2
	.byte	0x10
	.byte	0x4
	.4byte	.LASF3
	.byte	0x4
	.4byte	.LASF49
	.byte	0x3
	.4byte	.LASF5
	.byte	0x4
	.byte	0x22
	.byte	0x17
	.4byte	0x6b
	.byte	0x5
	.4byte	0x5a
	.byte	0x2
	.byte	0x1
	.byte	0x6
	.4byte	.LASF6
	.byte	0x2
	.byte	0x2
	.byte	0x5
	.4byte	.LASF7
	.byte	0x6
	.byte	0x4
	.byte	0x5
	.string	"int"
	.byte	0x3
	.4byte	.LASF8
	.byte	0x4
	.byte	0x2b
	.byte	0x18
	.4byte	0x2d
	.byte	0x5
	.4byte	0x80
	.byte	0x3
	.4byte	.LASF9
	.byte	0x4
	.byte	0x2e
	.byte	0x18
	.4byte	0xa2
	.byte	0x7
	.4byte	0x91
	.byte	0x2
	.byte	0x1
	.byte	0x8
	.4byte	.LASF10
	.byte	0x7
	.4byte	0xa2
	.byte	0x2
	.byte	0x2
	.byte	0x7
	.4byte	.LASF11
	.byte	0x2
	.byte	0x4
	.byte	0x7
	.4byte	.LASF12
	.byte	0x3
	.4byte	.LASF13
	.byte	0x4
	.byte	0x37
	.byte	0x19
	.4byte	0x40
	.byte	0x3
	.4byte	.LASF14
	.byte	0x2
	.byte	0x2a
	.byte	0xd
	.4byte	0x2d
	.byte	0x2
	.byte	0x2
	.byte	0x4
	.4byte	.LASF15
	.byte	0x2
	.byte	0x4
	.byte	0x4
	.4byte	.LASF16
	.byte	0x2
	.byte	0x8
	.byte	0x4
	.4byte	.LASF17
	.byte	0x8
	.4byte	.LASF50
	.byte	0x10
	.byte	0x1
	.byte	0x86
	.byte	0x8
	.4byte	0x104
	.byte	0x9
	.4byte	.LASF18
	.byte	0x1
	.byte	0x87
	.byte	0xe
	.4byte	0xbc
	.byte	0
	.byte	0
	.byte	0xa
	.4byte	.LASF51
	.byte	0x20
	.byte	0x10
	.byte	0x1
	.byte	0x8b
	.byte	0x8
	.4byte	0x12e
	.byte	0x9
	.4byte	.LASF19
	.byte	0x1
	.byte	0x8d
	.byte	0xe
	.4byte	0xbc
	.byte	0
	.byte	0xb
	.4byte	.LASF52
	.byte	0x1
	.byte	0x8f
	.byte	0x44
	.4byte	0x12e
	.byte	0x10
	.byte	0x10
	.byte	0
	.byte	0xc
	.byte	0x8
	.byte	0x10
	.4byte	0xe9
	.byte	0xd
	.4byte	.LASF20
	.byte	0x1
	.2byte	0x16e
	.byte	0x19
	.4byte	0x8c
	.byte	0xd
	.4byte	.LASF21
	.byte	0x1
	.2byte	0x16f
	.byte	0x19
	.4byte	0x8c
	.byte	0xd
	.4byte	.LASF22
	.byte	0x1
	.2byte	0x170
	.byte	0x18
	.4byte	0x66
	.byte	0xe
	.4byte	.LASF32
	.byte	0x1
	.2byte	0x175
	.byte	0x5
	.4byte	0x79
	.8byte	.LFB17417
	.8byte	.LFE17417-.LFB17417
	.byte	0x1
	.byte	0x9c
	.4byte	0x1ac
	.byte	0xf
	.4byte	.LASF23
	.byte	0x1
	.2byte	0x177
	.byte	0xd
	.4byte	0x80
	.4byte	.LLST21
	.byte	0x10
	.4byte	.LASF24
	.byte	0x1
	.2byte	0x178
	.byte	0xd
	.4byte	0x80
	.byte	0x1
	.byte	0x11
	.8byte	.LVL31
	.4byte	0x1ac
	.byte	0
	.byte	0x12
	.4byte	.LASF53
	.byte	0x1
	.byte	0xef
	.byte	0x5
	.4byte	.LASF54
	.4byte	0x79
	.8byte	.LFB17416
	.8byte	.LFE17416-.LFB17416
	.byte	0x1
	.byte	0x9c
	.4byte	0x2d8
	.byte	0x13
	.4byte	.LASF25
	.byte	0x1
	.byte	0xf3
	.byte	0xa
	.4byte	0x2d8
	.byte	0x3
	.byte	0x91
	.byte	0xa0,0x7f
	.byte	0x13
	.4byte	.LASF26
	.byte	0x1
	.byte	0xfc
	.byte	0x9
	.4byte	0x2e8
	.byte	0x3
	.byte	0x91
	.byte	0xa0,0x7b
	.byte	0x14
	.4byte	.LASF27
	.byte	0x1
	.2byte	0x10f
	.byte	0xd
	.4byte	0x2f8
	.byte	0x3
	.byte	0x91
	.byte	0xa0,0x5b
	.byte	0x14
	.4byte	.LASF28
	.byte	0x1
	.2byte	0x118
	.byte	0xd
	.4byte	0x2f8
	.byte	0x4
	.byte	0x91
	.byte	0xa0,0xbb,0x7f
	.byte	0x14
	.4byte	.LASF29
	.byte	0x1
	.2byte	0x11a
	.byte	0xe
	.4byte	0x309
	.byte	0x4
	.byte	0x91
	.byte	0xa0,0x5b
	.byte	0x9f
	.byte	0x14
	.4byte	.LASF30
	.byte	0x1
	.2byte	0x11b
	.byte	0xe
	.4byte	0x309
	.byte	0x5
	.byte	0x91
	.byte	0xa0,0xbb,0x7f
	.byte	0x9f
	.byte	0x15
	.4byte	.Ldebug_ranges0+0
	.4byte	0x276
	.byte	0x16
	.string	"i"
	.byte	0x1
	.2byte	0x110
	.byte	0x11
	.4byte	0x34
	.4byte	.LLST18
	.byte	0x17
	.8byte	.LBB30
	.8byte	.LBE30-.LBB30
	.byte	0xf
	.4byte	.LASF31
	.byte	0x1
	.2byte	0x111
	.byte	0xd
	.4byte	0x79
	.4byte	.LLST19
	.byte	0
	.byte	0
	.byte	0x15
	.4byte	.Ldebug_ranges0+0x30
	.4byte	0x28f
	.byte	0x16
	.string	"i"
	.byte	0x1
	.2byte	0x121
	.byte	0x11
	.4byte	0x34
	.4byte	.LLST20
	.byte	0
	.byte	0x18
	.8byte	.LVL27
	.4byte	0x630
	.4byte	0x2b5
	.byte	0x19
	.byte	0x1
	.byte	0x5a
	.byte	0x4
	.byte	0x91
	.byte	0xa0,0xbb,0x7f
	.byte	0x19
	.byte	0x1
	.byte	0x5b
	.byte	0x1
	.byte	0x30
	.byte	0x19
	.byte	0x1
	.byte	0x5c
	.byte	0x3
	.byte	0xa
	.2byte	0x1000
	.byte	0
	.byte	0x1a
	.8byte	.LVL28
	.4byte	0x30f
	.byte	0x19
	.byte	0x1
	.byte	0x5a
	.byte	0x2
	.byte	0x79
	.byte	0
	.byte	0x19
	.byte	0x1
	.byte	0x5b
	.byte	0x3
	.byte	0x79
	.byte	0x80,0x20
	.byte	0x19
	.byte	0x1
	.byte	0x5c
	.byte	0x3
	.byte	0xa
	.2byte	0x1000
	.byte	0
	.byte	0
	.byte	0x1b
	.4byte	0xe9
	.4byte	0x2e8
	.byte	0x1c
	.4byte	0x40
	.byte	0x3
	.byte	0
	.byte	0x1b
	.4byte	0x79
	.4byte	0x2f8
	.byte	0x1c
	.4byte	0x40
	.byte	0x7f
	.byte	0
	.byte	0x1d
	.4byte	0x104
	.byte	0x10
	.4byte	0x309
	.byte	0x1c
	.4byte	0x40
	.byte	0x7f
	.byte	0
	.byte	0x1e
	.byte	0x8
	.4byte	0x104
	.byte	0x1f
	.4byte	.LASF33
	.byte	0x1
	.byte	0x9c
	.byte	0x6
	.4byte	.LASF55
	.8byte	.LFB17415
	.8byte	.LFE17415-.LFB17415
	.byte	0x1
	.byte	0x9c
	.4byte	0x46e
	.byte	0x20
	.string	"dst"
	.byte	0x1
	.byte	0x9c
	.byte	0x1d
	.4byte	0x46e
	.4byte	.LLST6
	.byte	0x20
	.string	"src"
	.byte	0x1
	.byte	0x9c
	.byte	0x31
	.4byte	0x474
	.4byte	.LLST7
	.byte	0x21
	.4byte	.LASF34
	.byte	0x1
	.byte	0x9c
	.byte	0x3d
	.4byte	0x34
	.4byte	.LLST8
	.byte	0x17
	.8byte	.LBB18
	.8byte	.LBE18-.LBB18
	.byte	0x22
	.4byte	.LASF35
	.byte	0x1
	.byte	0xb0
	.byte	0x10
	.4byte	0x34
	.4byte	.LLST9
	.byte	0x23
	.4byte	.LASF45
	.byte	0x1
	.byte	0xb2
	.byte	0x14
	.4byte	0x47a
	.byte	0x24
	.4byte	0x605
	.8byte	.LBB19
	.8byte	.LBE19-.LBB19
	.byte	0x1
	.byte	0xb0
	.byte	0x2d
	.4byte	0x3bd
	.byte	0x25
	.4byte	0x61a
	.4byte	.LLST10
	.byte	0x26
	.4byte	0x624
	.4byte	.LLST9
	.byte	0
	.byte	0x24
	.4byte	0x5d7
	.8byte	.LBB21
	.8byte	.LBE21-.LBB21
	.byte	0x1
	.byte	0xb7
	.byte	0x1b
	.4byte	0x414
	.byte	0x25
	.4byte	0x5f8
	.4byte	.LLST12
	.byte	0x25
	.4byte	0x5ed
	.4byte	.LLST13
	.byte	0x27
	.4byte	0x605
	.8byte	.LBB23
	.8byte	.LBE23-.LBB23
	.byte	0x2
	.2byte	0x1fd
	.byte	0x1
	.byte	0x25
	.4byte	0x61a
	.4byte	.LLST14
	.byte	0x28
	.4byte	0x624
	.byte	0
	.byte	0
	.byte	0x29
	.4byte	0x5a2
	.8byte	.LBB25
	.8byte	.LBE25-.LBB25
	.byte	0x1
	.byte	0xb8
	.byte	0x14
	.byte	0x25
	.4byte	0x5ca
	.4byte	.LLST15
	.byte	0x2a
	.4byte	0x5bf
	.byte	0x25
	.4byte	0x5b4
	.4byte	.LLST16
	.byte	0x27
	.4byte	0x605
	.8byte	.LBB27
	.8byte	.LBE27-.LBB27
	.byte	0x2
	.2byte	0x1fe
	.byte	0x1
	.byte	0x25
	.4byte	0x61a
	.4byte	.LLST17
	.byte	0x28
	.4byte	0x624
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0x1e
	.byte	0x8
	.4byte	0x91
	.byte	0x1e
	.byte	0x8
	.4byte	0x9d
	.byte	0x2b
	.4byte	.LASF56
	.4byte	0xa2
	.4byte	0x492
	.byte	0x2c
	.byte	0x8
	.byte	0x92
	.byte	0xa2,0x38
	.byte	0
	.byte	0x40
	.byte	0x1e
	.byte	0x31
	.byte	0x1c
	.byte	0
	.byte	0x2d
	.4byte	.LASF36
	.byte	0x1
	.byte	0x13
	.byte	0x7
	.4byte	0x501
	.8byte	.LFB17414
	.8byte	.LFE17414-.LFB17414
	.byte	0x1
	.byte	0x9c
	.4byte	0x501
	.byte	0x2e
	.4byte	.LASF37
	.byte	0x1
	.byte	0x13
	.byte	0x14
	.4byte	0x501
	.byte	0x1
	.byte	0x5a
	.byte	0x20
	.string	"src"
	.byte	0x1
	.byte	0x13
	.byte	0x26
	.4byte	0x503
	.4byte	.LLST3
	.byte	0x21
	.4byte	.LASF38
	.byte	0x1
	.byte	0x13
	.byte	0x32
	.4byte	0x34
	.4byte	.LLST4
	.byte	0x22
	.4byte	.LASF39
	.byte	0x1
	.byte	0x14
	.byte	0x14
	.4byte	0x50a
	.4byte	.LLST5
	.byte	0x13
	.4byte	.LASF40
	.byte	0x1
	.byte	0x15
	.byte	0x1a
	.4byte	0x510
	.byte	0x1
	.byte	0x5b
	.byte	0
	.byte	0x2f
	.byte	0x8
	.byte	0x1e
	.byte	0x8
	.4byte	0x509
	.byte	0x30
	.byte	0x1e
	.byte	0x8
	.4byte	0xa2
	.byte	0x1e
	.byte	0x8
	.4byte	0xa9
	.byte	0x2d
	.4byte	.LASF41
	.byte	0x1
	.byte	0x8
	.byte	0x7
	.4byte	0x501
	.8byte	.LFB17413
	.8byte	.LFE17413-.LFB17413
	.byte	0x1
	.byte	0x9c
	.4byte	0x5a2
	.byte	0x2e
	.4byte	.LASF37
	.byte	0x1
	.byte	0x8
	.byte	0x14
	.4byte	0x501
	.byte	0x1
	.byte	0x5a
	.byte	0x20
	.string	"ch"
	.byte	0x1
	.byte	0x8
	.byte	0x1e
	.4byte	0x79
	.4byte	.LLST0
	.byte	0x21
	.4byte	.LASF38
	.byte	0x1
	.byte	0x8
	.byte	0x29
	.4byte	0x34
	.4byte	.LLST1
	.byte	0x13
	.4byte	.LASF42
	.byte	0x1
	.byte	0x9
	.byte	0x13
	.4byte	0xa2
	.byte	0x1
	.byte	0x5b
	.byte	0x13
	.4byte	.LASF39
	.byte	0x1
	.byte	0xa
	.byte	0x14
	.4byte	0x50a
	.byte	0x1
	.byte	0x5a
	.byte	0x17
	.8byte	.LBB17
	.8byte	.LBE17-.LBB17
	.byte	0x31
	.string	"i"
	.byte	0x1
	.byte	0xb
	.byte	0xe
	.4byte	0x79
	.4byte	.LLST2
	.byte	0
	.byte	0
	.byte	0x32
	.4byte	.LASF57
	.byte	0x2
	.2byte	0x1fe
	.byte	0x1
	.4byte	.LASF58
	.byte	0x3
	.4byte	0x5d7
	.byte	0x33
	.string	"a"
	.byte	0x2
	.2byte	0x1fe
	.byte	0x1
	.4byte	0x46e
	.byte	0x33
	.string	"b"
	.byte	0x2
	.2byte	0x1fe
	.byte	0x1
	.4byte	0x47a
	.byte	0x33
	.string	"vl"
	.byte	0x2
	.2byte	0x1fe
	.byte	0x1
	.4byte	0xc8
	.byte	0
	.byte	0x34
	.4byte	.LASF43
	.byte	0x2
	.2byte	0x1fd
	.byte	0x1
	.4byte	.LASF59
	.4byte	0x47a
	.byte	0x3
	.4byte	0x605
	.byte	0x33
	.string	"a"
	.byte	0x2
	.2byte	0x1fd
	.byte	0x1
	.4byte	0x474
	.byte	0x33
	.string	"vl"
	.byte	0x2
	.2byte	0x1fd
	.byte	0x1
	.4byte	0xc8
	.byte	0
	.byte	0x35
	.4byte	.LASF44
	.byte	0x2
	.byte	0x89
	.byte	0x1
	.4byte	.LASF60
	.4byte	0xc8
	.byte	0x3
	.4byte	0x630
	.byte	0x36
	.string	"a"
	.byte	0x2
	.byte	0x89
	.byte	0x1
	.4byte	0xc8
	.byte	0x37
	.string	"vl"
	.byte	0x2
	.byte	0x89
	.byte	0x1
	.4byte	0xc8
	.byte	0
	.byte	0x38
	.4byte	.LASF41
	.4byte	.LASF61
	.byte	0
	.section	.debug_abbrev,"",@progbits
.Ldebug_abbrev0:
	.byte	0x1
	.byte	0x11
	.byte	0x1
	.byte	0x25
	.byte	0xe
	.byte	0x13
	.byte	0xb
	.byte	0x3
	.byte	0xe
	.byte	0x1b
	.byte	0xe
	.byte	0x11
	.byte	0x1
	.byte	0x12
	.byte	0x7
	.byte	0x10
	.byte	0x17
	.byte	0
	.byte	0
	.byte	0x2
	.byte	0x24
	.byte	0
	.byte	0xb
	.byte	0xb
	.byte	0x3e
	.byte	0xb
	.byte	0x3
	.byte	0xe
	.byte	0
	.byte	0
	.byte	0x3
	.byte	0x16
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x4
	.byte	0x3b
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0
	.byte	0
	.byte	0x5
	.byte	0x35
	.byte	0
	.byte	0x49
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x6
	.byte	0x24
	.byte	0
	.byte	0xb
	.byte	0xb
	.byte	0x3e
	.byte	0xb
	.byte	0x3
	.byte	0x8
	.byte	0
	.byte	0
	.byte	0x7
	.byte	0x26
	.byte	0
	.byte	0x49
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x8
	.byte	0x13
	.byte	0x1
	.byte	0x3
	.byte	0xe
	.byte	0xb
	.byte	0xb
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x9
	.byte	0xd
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x38
	.byte	0xb
	.byte	0
	.byte	0
	.byte	0xa
	.byte	0x13
	.byte	0x1
	.byte	0x3
	.byte	0xe
	.byte	0xb
	.byte	0xb
	.byte	0x88,0x1
	.byte	0xb
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0xb
	.byte	0xd
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x88,0x1
	.byte	0xb
	.byte	0x38
	.byte	0xb
	.byte	0
	.byte	0
	.byte	0xc
	.byte	0xf
	.byte	0
	.byte	0xb
	.byte	0xb
	.byte	0x88,0x1
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0xd
	.byte	0x34
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0x5
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x3f
	.byte	0x19
	.byte	0x3c
	.byte	0x19
	.byte	0
	.byte	0
	.byte	0xe
	.byte	0x2e
	.byte	0x1
	.byte	0x3f
	.byte	0x19
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0x5
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x11
	.byte	0x1
	.byte	0x12
	.byte	0x7
	.byte	0x40
	.byte	0x18
	.byte	0x97,0x42
	.byte	0x19
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0xf
	.byte	0x34
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0x5
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x2
	.byte	0x17
	.byte	0
	.byte	0
	.byte	0x10
	.byte	0x34
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0x5
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x1c
	.byte	0xb
	.byte	0
	.byte	0
	.byte	0x11
	.byte	0x89,0x82,0x1
	.byte	0
	.byte	0x11
	.byte	0x1
	.byte	0x31
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x12
	.byte	0x2e
	.byte	0x1
	.byte	0x3f
	.byte	0x19
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x6e
	.byte	0xe
	.byte	0x49
	.byte	0x13
	.byte	0x11
	.byte	0x1
	.byte	0x12
	.byte	0x7
	.byte	0x40
	.byte	0x18
	.byte	0x97,0x42
	.byte	0x19
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x13
	.byte	0x34
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x2
	.byte	0x18
	.byte	0
	.byte	0
	.byte	0x14
	.byte	0x34
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0x5
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x2
	.byte	0x18
	.byte	0
	.byte	0
	.byte	0x15
	.byte	0xb
	.byte	0x1
	.byte	0x55
	.byte	0x17
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x16
	.byte	0x34
	.byte	0
	.byte	0x3
	.byte	0x8
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0x5
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x2
	.byte	0x17
	.byte	0
	.byte	0
	.byte	0x17
	.byte	0xb
	.byte	0x1
	.byte	0x11
	.byte	0x1
	.byte	0x12
	.byte	0x7
	.byte	0
	.byte	0
	.byte	0x18
	.byte	0x89,0x82,0x1
	.byte	0x1
	.byte	0x11
	.byte	0x1
	.byte	0x31
	.byte	0x13
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x19
	.byte	0x8a,0x82,0x1
	.byte	0
	.byte	0x2
	.byte	0x18
	.byte	0x91,0x42
	.byte	0x18
	.byte	0
	.byte	0
	.byte	0x1a
	.byte	0x89,0x82,0x1
	.byte	0x1
	.byte	0x11
	.byte	0x1
	.byte	0x31
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x1b
	.byte	0x1
	.byte	0x1
	.byte	0x49
	.byte	0x13
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x1c
	.byte	0x21
	.byte	0
	.byte	0x49
	.byte	0x13
	.byte	0x2f
	.byte	0xb
	.byte	0
	.byte	0
	.byte	0x1d
	.byte	0x1
	.byte	0x1
	.byte	0x49
	.byte	0x13
	.byte	0x88,0x1
	.byte	0xb
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x1e
	.byte	0xf
	.byte	0
	.byte	0xb
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x1f
	.byte	0x2e
	.byte	0x1
	.byte	0x3f
	.byte	0x19
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x6e
	.byte	0xe
	.byte	0x11
	.byte	0x1
	.byte	0x12
	.byte	0x7
	.byte	0x40
	.byte	0x18
	.byte	0x97,0x42
	.byte	0x19
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x20
	.byte	0x5
	.byte	0
	.byte	0x3
	.byte	0x8
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x2
	.byte	0x17
	.byte	0
	.byte	0
	.byte	0x21
	.byte	0x5
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x2
	.byte	0x17
	.byte	0
	.byte	0
	.byte	0x22
	.byte	0x34
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x2
	.byte	0x17
	.byte	0
	.byte	0
	.byte	0x23
	.byte	0x34
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x24
	.byte	0x1d
	.byte	0x1
	.byte	0x31
	.byte	0x13
	.byte	0x11
	.byte	0x1
	.byte	0x12
	.byte	0x7
	.byte	0x58
	.byte	0xb
	.byte	0x59
	.byte	0xb
	.byte	0x57
	.byte	0xb
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x25
	.byte	0x5
	.byte	0
	.byte	0x31
	.byte	0x13
	.byte	0x2
	.byte	0x17
	.byte	0
	.byte	0
	.byte	0x26
	.byte	0x34
	.byte	0
	.byte	0x31
	.byte	0x13
	.byte	0x2
	.byte	0x17
	.byte	0
	.byte	0
	.byte	0x27
	.byte	0x1d
	.byte	0x1
	.byte	0x31
	.byte	0x13
	.byte	0x11
	.byte	0x1
	.byte	0x12
	.byte	0x7
	.byte	0x58
	.byte	0xb
	.byte	0x59
	.byte	0x5
	.byte	0x57
	.byte	0xb
	.byte	0
	.byte	0
	.byte	0x28
	.byte	0x34
	.byte	0
	.byte	0x31
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x29
	.byte	0x1d
	.byte	0x1
	.byte	0x31
	.byte	0x13
	.byte	0x11
	.byte	0x1
	.byte	0x12
	.byte	0x7
	.byte	0x58
	.byte	0xb
	.byte	0x59
	.byte	0xb
	.byte	0x57
	.byte	0xb
	.byte	0
	.byte	0
	.byte	0x2a
	.byte	0x5
	.byte	0
	.byte	0x31
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x2b
	.byte	0x1
	.byte	0x1
	.byte	0x3
	.byte	0xe
	.byte	0x87,0x42
	.byte	0x19
	.byte	0x49
	.byte	0x13
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x2c
	.byte	0x21
	.byte	0
	.byte	0x2f
	.byte	0x18
	.byte	0
	.byte	0
	.byte	0x2d
	.byte	0x2e
	.byte	0x1
	.byte	0x3f
	.byte	0x19
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x11
	.byte	0x1
	.byte	0x12
	.byte	0x7
	.byte	0x40
	.byte	0x18
	.byte	0x97,0x42
	.byte	0x19
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x2e
	.byte	0x5
	.byte	0
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x2
	.byte	0x18
	.byte	0
	.byte	0
	.byte	0x2f
	.byte	0xf
	.byte	0
	.byte	0xb
	.byte	0xb
	.byte	0
	.byte	0
	.byte	0x30
	.byte	0x26
	.byte	0
	.byte	0
	.byte	0
	.byte	0x31
	.byte	0x34
	.byte	0
	.byte	0x3
	.byte	0x8
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0x2
	.byte	0x17
	.byte	0
	.byte	0
	.byte	0x32
	.byte	0x2e
	.byte	0x1
	.byte	0x3f
	.byte	0x19
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0x5
	.byte	0x39
	.byte	0xb
	.byte	0x6e
	.byte	0xe
	.byte	0x20
	.byte	0xb
	.byte	0x34
	.byte	0x19
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x33
	.byte	0x5
	.byte	0
	.byte	0x3
	.byte	0x8
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0x5
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x34
	.byte	0x2e
	.byte	0x1
	.byte	0x3f
	.byte	0x19
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0x5
	.byte	0x39
	.byte	0xb
	.byte	0x6e
	.byte	0xe
	.byte	0x49
	.byte	0x13
	.byte	0x20
	.byte	0xb
	.byte	0x34
	.byte	0x19
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x35
	.byte	0x2e
	.byte	0x1
	.byte	0x3f
	.byte	0x19
	.byte	0x3
	.byte	0xe
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x6e
	.byte	0xe
	.byte	0x49
	.byte	0x13
	.byte	0x20
	.byte	0xb
	.byte	0x34
	.byte	0x19
	.byte	0x1
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x36
	.byte	0x5
	.byte	0
	.byte	0x3
	.byte	0x8
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x37
	.byte	0x34
	.byte	0
	.byte	0x3
	.byte	0x8
	.byte	0x3a
	.byte	0xb
	.byte	0x3b
	.byte	0xb
	.byte	0x39
	.byte	0xb
	.byte	0x49
	.byte	0x13
	.byte	0
	.byte	0
	.byte	0x38
	.byte	0x2e
	.byte	0
	.byte	0x3f
	.byte	0x19
	.byte	0x3c
	.byte	0x19
	.byte	0x6e
	.byte	0xe
	.byte	0x3
	.byte	0xe
	.byte	0
	.byte	0
	.byte	0
	.section	.debug_loc,"",@progbits
.Ldebug_loc0:
.LLST21:
	.8byte	.LVL30-.Ltext0
	.8byte	.LVL31-.Ltext0
	.2byte	0x2
	.byte	0x30
	.byte	0x9f
	.8byte	.LVL31-.Ltext0
	.8byte	.LFE17417-.Ltext0
	.2byte	0x1
	.byte	0x5a
	.8byte	0
	.8byte	0
.LLST18:
	.8byte	.LVL22-.Ltext0
	.8byte	.LVL23-.Ltext0
	.2byte	0x2
	.byte	0x30
	.byte	0x9f
	.8byte	0
	.8byte	0
.LLST19:
	.8byte	.LVL24-.Ltext0
	.8byte	.LVL25-.Ltext0
	.2byte	0x1
	.byte	0x5f
	.8byte	.LVL25-.Ltext0
	.8byte	.LVL26-.Ltext0
	.2byte	0x2
	.byte	0x7d
	.byte	0
	.8byte	.LVL26-.Ltext0
	.8byte	.LVL27-1-.Ltext0
	.2byte	0x2
	.byte	0x7d
	.byte	0x7c
	.8byte	0
	.8byte	0
.LLST20:
	.8byte	.LVL28-.Ltext0
	.8byte	.LVL29-.Ltext0
	.2byte	0x2
	.byte	0x30
	.byte	0x9f
	.8byte	0
	.8byte	0
.LLST6:
	.8byte	.LVL11-.Ltext0
	.8byte	.LVL19-.Ltext0
	.2byte	0x1
	.byte	0x5a
	.8byte	.LVL19-.Ltext0
	.8byte	.LFE17415-.Ltext0
	.2byte	0x1
	.byte	0x5a
	.8byte	0
	.8byte	0
.LLST7:
	.8byte	.LVL11-.Ltext0
	.8byte	.LVL18-.Ltext0
	.2byte	0x1
	.byte	0x5b
	.8byte	.LVL18-.Ltext0
	.8byte	.LFE17415-.Ltext0
	.2byte	0x1
	.byte	0x5b
	.8byte	0
	.8byte	0
.LLST8:
	.8byte	.LVL11-.Ltext0
	.8byte	.LVL20-.Ltext0
	.2byte	0x1
	.byte	0x5c
	.8byte	.LVL20-.Ltext0
	.8byte	.LFE17415-.Ltext0
	.2byte	0x1
	.byte	0x5c
	.8byte	0
	.8byte	0
.LLST9:
	.8byte	.LVL13-.Ltext0
	.8byte	.LVL21-.Ltext0
	.2byte	0x1
	.byte	0x5f
	.8byte	0
	.8byte	0
.LLST10:
	.8byte	.LVL12-.Ltext0
	.8byte	.LVL13-.Ltext0
	.2byte	0x1
	.byte	0x5c
	.8byte	0
	.8byte	0
.LLST12:
	.8byte	.LVL13-.Ltext0
	.8byte	.LVL15-.Ltext0
	.2byte	0x1
	.byte	0x5f
	.8byte	0
	.8byte	0
.LLST13:
	.8byte	.LVL13-.Ltext0
	.8byte	.LVL15-.Ltext0
	.2byte	0x1
	.byte	0x5b
	.8byte	0
	.8byte	0
.LLST14:
	.8byte	.LVL13-.Ltext0
	.8byte	.LVL14-.Ltext0
	.2byte	0x1
	.byte	0x5f
	.8byte	0
	.8byte	0
.LLST15:
	.8byte	.LVL15-.Ltext0
	.8byte	.LVL17-.Ltext0
	.2byte	0x1
	.byte	0x5f
	.8byte	0
	.8byte	0
.LLST16:
	.8byte	.LVL15-.Ltext0
	.8byte	.LVL17-.Ltext0
	.2byte	0x1
	.byte	0x5a
	.8byte	0
	.8byte	0
.LLST17:
	.8byte	.LVL15-.Ltext0
	.8byte	.LVL16-.Ltext0
	.2byte	0x1
	.byte	0x5f
	.8byte	0
	.8byte	0
.LLST3:
	.8byte	.LVL5-.Ltext0
	.8byte	.LVL7-.Ltext0
	.2byte	0x1
	.byte	0x5b
	.8byte	.LVL7-.Ltext0
	.8byte	.LFE17414-.Ltext0
	.2byte	0x4
	.byte	0xf3
	.byte	0x1
	.byte	0x5b
	.byte	0x9f
	.8byte	0
	.8byte	0
.LLST4:
	.8byte	.LVL5-.Ltext0
	.8byte	.LVL6-.Ltext0
	.2byte	0x1
	.byte	0x5c
	.8byte	.LVL6-.Ltext0
	.8byte	.LFE17414-.Ltext0
	.2byte	0x4
	.byte	0xf3
	.byte	0x1
	.byte	0x5c
	.byte	0x9f
	.8byte	0
	.8byte	0
.LLST5:
	.8byte	.LVL5-.Ltext0
	.8byte	.LVL7-.Ltext0
	.2byte	0x1
	.byte	0x5a
	.8byte	.LVL7-.Ltext0
	.8byte	.LVL10-.Ltext0
	.2byte	0x1
	.byte	0x5f
	.8byte	0
	.8byte	0
.LLST0:
	.8byte	.LVL0-.Ltext0
	.8byte	.LVL1-.Ltext0
	.2byte	0x1
	.byte	0x5b
	.8byte	.LVL1-.Ltext0
	.8byte	.LFE17413-.Ltext0
	.2byte	0x4
	.byte	0xf3
	.byte	0x1
	.byte	0x5b
	.byte	0x9f
	.8byte	0
	.8byte	0
.LLST1:
	.8byte	.LVL0-.Ltext0
	.8byte	.LVL2-.Ltext0
	.2byte	0x1
	.byte	0x5c
	.8byte	.LVL2-.Ltext0
	.8byte	.LFE17413-.Ltext0
	.2byte	0x4
	.byte	0xf3
	.byte	0x1
	.byte	0x5c
	.byte	0x9f
	.8byte	0
	.8byte	0
.LLST2:
	.8byte	.LVL1-.Ltext0
	.8byte	.LVL2-.Ltext0
	.2byte	0x2
	.byte	0x30
	.byte	0x9f
	.8byte	.LVL2-.Ltext0
	.8byte	.LVL3-.Ltext0
	.2byte	0x6
	.byte	0x7f
	.byte	0
	.byte	0x7a
	.byte	0
	.byte	0x1c
	.byte	0x9f
	.8byte	.LVL3-.Ltext0
	.8byte	.LVL4-.Ltext0
	.2byte	0x8
	.byte	0x7f
	.byte	0
	.byte	0x7a
	.byte	0
	.byte	0x1c
	.byte	0x23
	.byte	0x1
	.byte	0x9f
	.8byte	0
	.8byte	0
	.section	.debug_aranges,"",@progbits
	.4byte	0x2c
	.2byte	0x2
	.4byte	.Ldebug_info0
	.byte	0x8
	.byte	0
	.2byte	0
	.2byte	0
	.8byte	.Ltext0
	.8byte	.Letext0-.Ltext0
	.8byte	0
	.8byte	0
	.section	.debug_ranges,"",@progbits
.Ldebug_ranges0:
	.8byte	.LBB29-.Ltext0
	.8byte	.LBE29-.Ltext0
	.8byte	.LBB31-.Ltext0
	.8byte	.LBE31-.Ltext0
	.8byte	0
	.8byte	0
	.8byte	.LBB32-.Ltext0
	.8byte	.LBE32-.Ltext0
	.8byte	.LBB33-.Ltext0
	.8byte	.LBE33-.Ltext0
	.8byte	.LBB34-.Ltext0
	.8byte	.LBE34-.Ltext0
	.8byte	0
	.8byte	0
	.section	.debug_line,"",@progbits
.Ldebug_line0:
	.section	.debug_str,"MS",@progbits,1
.LASF57:
	.string	"vse8_v_u8m8"
.LASF23:
	.string	"result"
.LASF16:
	.string	"float"
.LASF49:
	.string	"decltype(nullptr)"
.LASF44:
	.string	"vsetvl_e8m8"
.LASF37:
	.string	"dest"
.LASF5:
	.string	"int8_t"
.LASF13:
	.string	"uint64_t"
.LASF7:
	.string	"short int"
.LASF4:
	.string	"size_t"
.LASF54:
	.string	"_Z13run_base_testv"
.LASF33:
	.string	"vector_memcpy"
.LASF32:
	.string	"main"
.LASF36:
	.string	"memcpy"
.LASF39:
	.string	"dest_uc"
.LASF35:
	.string	"copied_per_iter"
.LASF8:
	.string	"int64_t"
.LASF18:
	.string	"value"
.LASF61:
	.string	"__builtin_memset"
.LASF9:
	.string	"uint8_t"
.LASF56:
	.string	"__vector([128,128]) unsigned char"
.LASF3:
	.string	"long double"
.LASF2:
	.string	"long long int"
.LASF42:
	.string	"ch_uc"
.LASF41:
	.string	"memset"
.LASF0:
	.string	"long int"
.LASF14:
	.string	"word_type"
.LASF22:
	.string	"finished"
.LASF34:
	.string	"num_bytes"
.LASF46:
	.string	"GNU C++14 10.1.0 -march=rv64imv -mabi=lp64 -mtune=rocket -march=rv64imv_zvamo_zvlsseg -g -O1 -fno-inline-functions -ffreestanding"
.LASF59:
	.string	"_Z11vle8_v_u8m8PKhl"
.LASF10:
	.string	"unsigned char"
.LASF51:
	.string	"Element"
.LASF6:
	.string	"signed char"
.LASF12:
	.string	"unsigned int"
.LASF28:
	.string	"dest_array"
.LASF11:
	.string	"short unsigned int"
.LASF48:
	.string	"/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/build/gcc-rv64imv/vector_memcpy_pointers"
.LASF50:
	.string	"Base"
.LASF25:
	.string	"bases"
.LASF19:
	.string	"expected_base_value"
.LASF45:
	.string	"data"
.LASF21:
	.string	"outputSucceeded"
.LASF30:
	.string	"dst_ptr"
.LASF20:
	.string	"outputAttempted"
.LASF1:
	.string	"long unsigned int"
.LASF15:
	.string	"__fp16"
.LASF17:
	.string	"double"
.LASF40:
	.string	"src_uc"
.LASF58:
	.string	"_Z11vse8_v_u8m8Ph_vuint8m8_tl"
.LASF38:
	.string	"count"
.LASF55:
	.string	"_Z13vector_memcpyPhPKhm"
.LASF24:
	.string	"attempted"
.LASF60:
	.string	"_Z11vsetvl_e8m8l"
.LASF29:
	.string	"src_ptr"
.LASF31:
	.string	"index"
.LASF47:
	.string	"/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/vector_memcpy_pointers/vector_memcpy_pointers.cpp"
.LASF43:
	.string	"vle8_v_u8m8"
.LASF26:
	.string	"indices"
.LASF52:
	.string	"base_ptr"
.LASF53:
	.string	"run_base_test"
.LASF27:
	.string	"source_array"
	.ident	"GCC: (GNU) 10.1.0"
