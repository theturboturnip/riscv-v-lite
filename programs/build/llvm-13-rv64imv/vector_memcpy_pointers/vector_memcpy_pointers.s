	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p0_m2p0_v0p10_zvlsseg0p10"
	.file	"vector_memcpy_pointers.cpp"
	.file	1 "/usr/lib/llvm-13/lib/clang/13.0.1/include" "stddef.h"
	.file	2 "/usr/lib/llvm-13/lib/clang/13.0.1/include" "stdint.h"
	.file	3 "/usr/lib/llvm-13/lib/clang/13.0.1/include" "riscv_vector.h"
	.file	4 "/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs" "vector_memcpy_pointers/vector_memcpy_pointers.cpp"
	.globl	memset
	.p2align	2
	.type	memset,@function
memset:
.Lfunc_begin0:
	.loc	4 8 0
	.cfi_sections .debug_frame
	.cfi_startproc
	.loc	4 11 5 prologue_end
	beqz	a2, .LBB0_3
.Ltmp0:
	.loc	4 0 5 is_stmt 0
	mv	a3, a0
.Ltmp1:
.LBB0_2:
	.loc	4 12 24 is_stmt 1
	sb	a1, 0(a3)
.Ltmp2:
	.loc	4 11 23
	addi	a2, a2, -1
	addi	a3, a3, 1
.Ltmp3:
	.loc	4 11 5 is_stmt 0
	bnez	a2, .LBB0_2
.Ltmp4:
.LBB0_3:
	.loc	4 15 5 is_stmt 1
	ret
.Ltmp5:
.Lfunc_end0:
	.size	memset, .Lfunc_end0-memset
	.cfi_endproc

	.globl	memcpy
	.p2align	2
	.type	memcpy,@function
memcpy:
.Lfunc_begin1:
	.loc	4 19 0
	.cfi_startproc
	.loc	4 22 5 prologue_end
	beqz	a2, .LBB1_3
.Ltmp6:
	.loc	4 0 5 is_stmt 0
	mv	a3, a0
.Ltmp7:
.LBB1_2:
	.loc	4 23 20 is_stmt 1
	lb	a4, 0(a1)
	.loc	4 23 18 is_stmt 0
	sb	a4, 0(a3)
	.loc	4 24 16 is_stmt 1
	addi	a3, a3, 1
.Ltmp8:
	.loc	4 26 14
	addi	a2, a2, -1
.Ltmp9:
	.loc	4 25 15
	addi	a1, a1, 1
.Ltmp10:
	.loc	4 22 5
	bnez	a2, .LBB1_2
.Ltmp11:
.LBB1_3:
	.loc	4 28 5
	ret
.Ltmp12:
.Lfunc_end1:
	.size	memcpy, .Lfunc_end1-memcpy
	.cfi_endproc

	.globl	_Z13vector_memcpyPhPKhm
	.p2align	2
	.type	_Z13vector_memcpyPhPKhm,@function
_Z13vector_memcpyPhPKhm:
.Lfunc_begin2:
	.loc	4 151 0
	.cfi_startproc
	.loc	4 170 5 prologue_end
	beqz	a2, .LBB2_2
.Ltmp13:
.LBB2_1:
	.loc	4 171 34
	vsetvli	a3, a2, e8, m8, ta, mu
.Ltmp14:
	.loc	4 178 16
	vle8.v	v8, (a1)
.Ltmp15:
	.loc	4 179 9
	vse8.v	v8, (a0)
	.loc	4 182 13
	add	a1, a1, a3
.Ltmp16:
	.loc	4 184 19
	sub	a2, a2, a3
.Ltmp17:
	.loc	4 183 13
	add	a0, a0, a3
.Ltmp18:
	.loc	4 170 5
	bnez	a2, .LBB2_1
.Ltmp19:
.LBB2_2:
	.loc	4 186 1
	ret
.Ltmp20:
.Lfunc_end2:
	.size	_Z13vector_memcpyPhPKhm, .Lfunc_end2-_Z13vector_memcpyPhPKhm
	.cfi_endproc

	.globl	_Z13run_base_testv
	.p2align	2
	.type	_Z13run_base_testv,@function
_Z13run_base_testv:
.Lfunc_begin3:
	.loc	4 188 0
	.cfi_startproc
	addi	sp, sp, -2032
	.cfi_def_cfa_offset 2032
	sd	ra, 2024(sp)
	.cfi_offset ra, -8
	lui	a0, 2
	addiw	a0, a0, -1936
	sub	sp, sp, a0
	.cfi_def_cfa_offset 8288
	mv	a0, zero
.Ltmp21:
	.loc	4 193 16 prologue_end
	lui	a1, 2
	addiw	a1, a1, 72
	add	a1, sp, a1
	sd	zero, 0(a1)
	lui	a1, 2
	addiw	a1, a1, 56
	add	a1, sp, a1
	sd	zero, 0(a1)
	lui	a1, 2
	addiw	a1, a1, 40
	add	a1, sp, a1
	sd	zero, 0(a1)
	lui	a1, 2
	addiw	a1, a1, 24
	add	a1, sp, a1
	sd	zero, 0(a1)
	lui	a1, 1863
	addiw	a1, a1, -271
	slli	a1, a1, 15
	addi	a1, a1, -1709
	slli	a1, a1, 13
	addi	a1, a1, -1207
	slli	a1, a1, 12
	addi	a1, a1, 1882
	lui	a2, 2
	addiw	a2, a2, 16
	add	a2, sp, a2
	sd	a1, 0(a2)
	lui	a1, 1033844
	addiw	a1, a1, -253
	slli	a1, a1, 12
	addi	a1, a1, 541
	slli	a1, a1, 13
	addi	a1, a1, 1579
	slli	a1, a1, 12
	addi	a1, a1, 1188
	lui	a2, 2
	addiw	a2, a2, 32
	add	a2, sp, a2
	sd	a1, 0(a2)
	lui	a1, 3301
	addiw	a1, a1, -205
	slli	a1, a1, 13
	addi	a1, a1, 1479
	slli	a1, a1, 14
	addi	a1, a1, 831
	slli	a1, a1, 12
	addi	a1, a1, -802
	lui	a2, 2
	addiw	a2, a2, 48
	add	a2, sp, a2
	sd	a1, 0(a2)
	lui	a1, 1045152
	addiw	a1, a1, -327
	slli	a1, a1, 14
	addi	a1, a1, -453
	slli	a1, a1, 12
	addi	a1, a1, -657
	slli	a1, a1, 13
	addi	a1, a1, 1687
	lui	a2, 2
	addiw	a2, a2, 64
	add	a2, sp, a2
	sd	a1, 0(a2)
.Ltmp22:
	.loc	4 222 5
	lui	a1, 1
	addiw	a1, a1, 32
	add	a1, sp, a1
	lui	a2, %hi(.L__const._Z13run_base_testv.indices)
	addi	a2, a2, %lo(.L__const._Z13run_base_testv.indices)
	lui	a3, 2
	addiw	a3, a3, 16
	add	a6, sp, a3
	addi	a4, zero, 512
.Ltmp23:
.LBB3_1:
	.loc	4 223 21
	add	a5, a0, a2
	lw	a5, 0(a5)
.Ltmp24:
	.loc	4 225 36
	slli	a5, a5, 4
	add	a5, a6, a5
	.loc	4 225 49 is_stmt 0
	ld	a3, 0(a5)
	.loc	4 224 25 is_stmt 1
	sd	a3, -16(a1)
	sd	a5, 0(a1)
.Ltmp25:
	.loc	4 222 26
	addi	a0, a0, 4
.Ltmp26:
	addi	a1, a1, 32
.Ltmp27:
	.loc	4 222 5 is_stmt 0
	bne	a0, a4, .LBB3_1
.Ltmp28:
	.loc	4 230 13 is_stmt 1
	addi	a0, sp, 16
	lui	a2, 1
	mv	a1, zero
	call	memset@plt
.Ltmp29:
	.loc	4 236 5
	addi	a0, sp, 16
	lui	a1, 1
	addiw	a1, a1, 16
	add	a1, sp, a1
	lui	a2, 1
	call	_Z13vector_memcpyPhPKhm
.Ltmp30:
	.loc	4 0 5 is_stmt 0
	mv	a0, zero
.Ltmp31:
	.loc	4 239 5 is_stmt 1
	addi	a1, sp, 32
	lui	a2, %hi(.L__const._Z13run_base_testv.indices)
	addi	a2, a2, %lo(.L__const._Z13run_base_testv.indices)
	lui	a3, 2
	addiw	a3, a3, 16
	add	a7, sp, a3
	addi	a6, zero, 512
.Ltmp32:
.LBB3_3:
	.loc	4 242 27
	ld	a5, 0(a1)
	.loc	4 242 37 is_stmt 0
	ld	a4, 0(a5)
	.loc	4 242 60
	ld	a3, -16(a1)
	bne	a4, a3, .LBB3_7
.Ltmp33:
	.loc	4 245 84 is_stmt 1
	add	a3, a0, a2
	lw	a3, 0(a3)
	.loc	4 245 78 is_stmt 0
	slli	a3, a3, 4
	add	a3, a7, a3
.Ltmp34:
	.loc	4 0 78
	bne	a5, a3, .LBB3_7
.Ltmp35:
	.loc	4 239 26 is_stmt 1
	addi	a0, a0, 4
.Ltmp36:
	addi	a1, a1, 32
.Ltmp37:
	.loc	4 239 5 is_stmt 0
	bne	a0, a6, .LBB3_3
.Ltmp38:
	.loc	4 0 5
	addi	a0, zero, 1
	j	.LBB3_8
.Ltmp39:
.LBB3_7:
	mv	a0, zero
.Ltmp40:
.LBB3_8:
	.loc	4 250 1 is_stmt 1
	lui	a1, 2
	addiw	a1, a1, -1936
	add	sp, sp, a1
	ld	ra, 2024(sp)
	addi	sp, sp, 2032
	ret
.Ltmp41:
.Lfunc_end3:
	.size	_Z13run_base_testv, .Lfunc_end3-_Z13run_base_testv
	.cfi_endproc

	.globl	main
	.p2align	2
	.type	main,@function
main:
.Lfunc_begin4:
	.loc	4 260 0
	.cfi_startproc
	addi	sp, sp, -16
	.cfi_def_cfa_offset 16
.Ltmp42:
	.loc	4 0 0 prologue_end
	sd	ra, 8(sp)
	.cfi_offset ra, -8
.Ltmp43:
	.loc	4 265 15
	call	_Z13run_base_testv
.Ltmp44:
	.loc	4 267 25
	lui	a1, %hi(outputAttempted)
	addi	a2, zero, 1
	sw	a2, %lo(outputAttempted)(a1)
	.loc	4 268 25
	lui	a1, %hi(outputSucceeded)
	sw	a0, %lo(outputSucceeded)(a1)
	.loc	4 269 5
	ld	ra, 8(sp)
	addi	sp, sp, 16
	ret
.Ltmp45:
.Lfunc_end4:
	.size	main, .Lfunc_end4-main
	.cfi_endproc

	.type	.L__const._Z13run_base_testv.indices,@object
	.section	.rodata,"a",@progbits
	.p2align	2
.L__const._Z13run_base_testv.indices:
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
	.size	.L__const._Z13run_base_testv.indices, 512

	.section	.debug_loc,"",@progbits
.Ldebug_loc0:
	.quad	.Lfunc_begin0-.Lfunc_begin0
	.quad	.Ltmp1-.Lfunc_begin0
	.half	1
	.byte	92
	.quad	0
	.quad	0
.Ldebug_loc1:
	.quad	.Lfunc_begin0-.Lfunc_begin0
	.quad	.Ltmp1-.Lfunc_begin0
	.half	3
	.byte	17
	.byte	0
	.byte	159
	.quad	.Ltmp1-.Lfunc_begin0
	.quad	.Ltmp2-.Lfunc_begin0
	.half	6
	.byte	125
	.byte	0
	.byte	122
	.byte	0
	.byte	28
	.byte	159
	.quad	.Ltmp2-.Lfunc_begin0
	.quad	.Ltmp3-.Lfunc_begin0
	.half	9
	.byte	125
	.byte	0
	.byte	122
	.byte	0
	.byte	28
	.byte	17
	.byte	1
	.byte	34
	.byte	159
	.quad	0
	.quad	0
.Ldebug_loc2:
	.quad	.Lfunc_begin1-.Lfunc_begin0
	.quad	.Ltmp7-.Lfunc_begin0
	.half	1
	.byte	91
	.quad	0
	.quad	0
.Ldebug_loc3:
	.quad	.Lfunc_begin1-.Lfunc_begin0
	.quad	.Ltmp7-.Lfunc_begin0
	.half	1
	.byte	90
	.quad	.Ltmp7-.Lfunc_begin0
	.quad	.Ltmp11-.Lfunc_begin0
	.half	1
	.byte	93
	.quad	0
	.quad	0
.Ldebug_loc4:
	.quad	.Ltmp14-.Lfunc_begin0
	.quad	.Ltmp19-.Lfunc_begin0
	.half	1
	.byte	93
	.quad	0
	.quad	0
.Ldebug_loc5:
	.quad	.Ltmp15-.Lfunc_begin0
	.quad	.Ltmp19-.Lfunc_begin0
	.half	2
	.byte	144
	.byte	104
	.quad	0
	.quad	0
.Ldebug_loc6:
	.quad	.Ltmp22-.Lfunc_begin0
	.quad	.Ltmp23-.Lfunc_begin0
	.half	2
	.byte	48
	.byte	159
	.quad	.Ltmp23-.Lfunc_begin0
	.quad	.Ltmp25-.Lfunc_begin0
	.half	6
	.byte	122
	.byte	0
	.byte	17
	.byte	4
	.byte	27
	.byte	159
	.quad	.Ltmp25-.Lfunc_begin0
	.quad	.Ltmp26-.Lfunc_begin0
	.half	9
	.byte	122
	.byte	0
	.byte	17
	.byte	4
	.byte	27
	.byte	17
	.byte	1
	.byte	34
	.byte	159
	.quad	0
	.quad	0
.Ldebug_loc7:
	.quad	.Ltmp29-.Lfunc_begin0
	.quad	.Lfunc_end3-.Lfunc_begin0
	.half	3
	.byte	114
	.byte	16
	.byte	159
	.quad	0
	.quad	0
.Ldebug_loc8:
	.quad	.Ltmp29-.Lfunc_begin0
	.quad	.Lfunc_end3-.Lfunc_begin0
	.half	4
	.byte	114
	.byte	144
	.byte	32
	.byte	159
	.quad	0
	.quad	0
.Ldebug_loc9:
	.quad	.Ltmp30-.Lfunc_begin0
	.quad	.Ltmp32-.Lfunc_begin0
	.half	2
	.byte	48
	.byte	159
	.quad	.Ltmp32-.Lfunc_begin0
	.quad	.Ltmp34-.Lfunc_begin0
	.half	6
	.byte	122
	.byte	0
	.byte	17
	.byte	4
	.byte	27
	.byte	159
	.quad	.Ltmp34-.Lfunc_begin0
	.quad	.Ltmp36-.Lfunc_begin0
	.half	9
	.byte	122
	.byte	0
	.byte	17
	.byte	4
	.byte	27
	.byte	17
	.byte	1
	.byte	34
	.byte	159
	.quad	0
	.quad	0
.Ldebug_loc10:
	.quad	.Ltmp43-.Lfunc_begin0
	.quad	.Ltmp44-.Lfunc_begin0
	.half	3
	.byte	17
	.byte	0
	.byte	159
	.quad	0
	.quad	0
	.section	.debug_abbrev,"",@progbits
	.byte	1
	.byte	17
	.byte	1
	.byte	37
	.byte	14
	.byte	19
	.byte	5
	.byte	3
	.byte	14
	.byte	16
	.byte	23
	.byte	27
	.byte	14
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	0
	.byte	0
	.byte	2
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	62
	.byte	11
	.byte	11
	.byte	11
	.byte	0
	.byte	0
	.byte	3
	.byte	15
	.byte	0
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	4
	.byte	38
	.byte	0
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	5
	.byte	22
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	0
	.byte	0
	.byte	6
	.byte	22
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	7
	.byte	1
	.byte	1
	.ascii	"\207B"
	.byte	25
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	8
	.byte	33
	.byte	0
	.byte	73
	.byte	19
	.byte	34
	.byte	13
	.byte	47
	.byte	24
	.byte	0
	.byte	0
	.byte	9
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	62
	.byte	11
	.byte	0
	.byte	0
	.byte	10
	.byte	19
	.byte	1
	.byte	54
	.byte	11
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	11
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	56
	.byte	11
	.byte	0
	.byte	0
	.byte	12
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.ascii	"\227B"
	.byte	25
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	13
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	14
	.byte	5
	.byte	0
	.byte	2
	.byte	23
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	15
	.byte	52
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	16
	.byte	11
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	0
	.byte	0
	.byte	17
	.byte	52
	.byte	0
	.byte	2
	.byte	23
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	18
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.ascii	"\227B"
	.byte	25
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	19
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.ascii	"\227B"
	.byte	25
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	20
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	21
	.ascii	"\211\202\001"
	.byte	0
	.byte	49
	.byte	19
	.byte	17
	.byte	1
	.byte	0
	.byte	0
	.byte	22
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.ascii	"\227B"
	.byte	25
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	23
	.byte	52
	.byte	0
	.byte	2
	.byte	23
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	24
	.byte	52
	.byte	0
	.byte	28
	.byte	13
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	25
	.byte	15
	.byte	0
	.byte	0
	.byte	0
	.byte	26
	.byte	38
	.byte	0
	.byte	0
	.byte	0
	.byte	27
	.byte	1
	.byte	1
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	28
	.byte	33
	.byte	0
	.byte	73
	.byte	19
	.byte	55
	.byte	11
	.byte	0
	.byte	0
	.byte	29
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	11
	.byte	0
	.byte	0
	.byte	0
	.section	.debug_info,"",@progbits
.Lcu_begin0:
	.word	.Ldebug_info_end0-.Ldebug_info_start0
.Ldebug_info_start0:
	.half	4
	.word	.debug_abbrev
	.byte	8
	.byte	1
	.word	.Linfo_string0
	.half	33
	.word	.Linfo_string1
	.word	.Lline_table_start0
	.word	.Linfo_string2
	.quad	.Lfunc_begin0
	.word	.Lfunc_end4-.Lfunc_begin0
	.byte	2
	.word	.Linfo_string3
	.byte	8
	.byte	1
	.byte	3
	.word	42
	.byte	3
	.word	59
	.byte	4
	.word	42
	.byte	5
	.word	75
	.word	.Linfo_string5
	.byte	1
	.byte	46
	.byte	2
	.word	.Linfo_string4
	.byte	7
	.byte	8
	.byte	3
	.word	87
	.byte	4
	.word	92
	.byte	5
	.word	42
	.word	.Linfo_string6
	.byte	2
	.byte	226
	.byte	3
	.word	92
	.byte	5
	.word	119
	.word	.Linfo_string9
	.byte	3
	.byte	99
	.byte	6
	.word	128
	.word	.Linfo_string8
	.byte	7

	.word	42
	.byte	8
	.word	149
	.byte	0
	.byte	8
	.byte	146
	.ascii	"\2428"
	.byte	0
	.byte	49
	.byte	27
	.byte	56
	.byte	30
	.byte	0
	.byte	9
	.word	.Linfo_string7
	.byte	8
	.byte	7
	.byte	3
	.word	161
	.byte	4
	.word	166
	.byte	10
	.byte	5
	.word	.Linfo_string12
	.byte	16
	.byte	4
	.byte	131
	.byte	16
	.byte	11
	.word	.Linfo_string10
	.word	189
	.byte	4
	.byte	132
	.byte	0
	.byte	0
	.byte	5
	.word	75
	.word	.Linfo_string11
	.byte	2
	.byte	98
	.byte	12
	.quad	.Lfunc_begin0
	.word	.Lfunc_end0-.Lfunc_begin0
	.byte	1
	.byte	82

	.word	.Linfo_string13
	.byte	4
	.byte	8
	.word	809

	.byte	13
	.byte	1
	.byte	90
	.word	.Linfo_string21
	.byte	4
	.byte	8
	.word	809
	.byte	13
	.byte	1
	.byte	91
	.word	.Linfo_string22
	.byte	4
	.byte	8
	.word	810
	.byte	14
	.word	.Ldebug_loc0
	.word	.Linfo_string23
	.byte	4
	.byte	8
	.word	64
	.byte	15
	.byte	3
	.byte	123
	.byte	0
	.byte	159
	.word	.Linfo_string24
	.byte	4
	.byte	9
	.word	42
	.byte	15
	.byte	1
	.byte	90
	.word	.Linfo_string25
	.byte	4
	.byte	10
	.word	49
	.byte	16
	.quad	.Lfunc_begin0
	.word	.Ltmp4-.Lfunc_begin0
	.byte	17
	.word	.Ldebug_loc1
	.word	.Linfo_string26
	.byte	4
	.byte	11
	.word	810
	.byte	0
	.byte	0
	.byte	12
	.quad	.Lfunc_begin1
	.word	.Lfunc_end1-.Lfunc_begin1
	.byte	1
	.byte	82

	.word	.Linfo_string14
	.byte	4
	.byte	19
	.word	809

	.byte	13
	.byte	1
	.byte	90
	.word	.Linfo_string21
	.byte	4
	.byte	19
	.word	809
	.byte	14
	.word	.Ldebug_loc2
	.word	.Linfo_string27
	.byte	4
	.byte	19
	.word	817
	.byte	13
	.byte	1
	.byte	92
	.word	.Linfo_string23
	.byte	4
	.byte	19
	.word	64
	.byte	17
	.word	.Ldebug_loc3
	.word	.Linfo_string25
	.byte	4
	.byte	20
	.word	49
	.byte	15
	.byte	1
	.byte	91
	.word	.Linfo_string28
	.byte	4
	.byte	21
	.word	54
	.byte	0
	.byte	18
	.quad	.Lfunc_begin2
	.word	.Lfunc_end2-.Lfunc_begin2
	.byte	1
	.byte	82

	.word	.Linfo_string15
	.word	.Linfo_string16
	.byte	4
	.byte	151

	.byte	13
	.byte	1
	.byte	90
	.word	.Linfo_string29
	.byte	4
	.byte	151
	.word	103
	.byte	13
	.byte	1
	.byte	91
	.word	.Linfo_string27
	.byte	4
	.byte	151
	.word	82
	.byte	13
	.byte	1
	.byte	92
	.word	.Linfo_string30
	.byte	4
	.byte	151
	.word	64
	.byte	16
	.quad	.Ltmp13
	.word	.Ltmp18-.Ltmp13
	.byte	17
	.word	.Ldebug_loc4
	.word	.Linfo_string31
	.byte	4
	.byte	171
	.word	64
	.byte	17
	.word	.Ldebug_loc5
	.word	.Linfo_string32
	.byte	4
	.byte	173
	.word	108
	.byte	0
	.byte	0
	.byte	19
	.quad	.Lfunc_begin3
	.word	.Lfunc_end3-.Lfunc_begin3
	.byte	1
	.byte	82

	.word	.Linfo_string17
	.word	.Linfo_string18
	.byte	4
	.byte	188
	.word	810

	.byte	15
	.byte	4
	.byte	145
	.asciz	"\220\300"
	.word	.Linfo_string33
	.byte	4
	.byte	193
	.word	823
	.byte	15
	.byte	3
	.byte	145
	.ascii	"\220 "
	.word	.Linfo_string34
	.byte	4
	.byte	221
	.word	835
	.byte	15
	.byte	2
	.byte	145
	.byte	16
	.word	.Linfo_string38
	.byte	4
	.byte	230
	.word	835
	.byte	17
	.word	.Ldebug_loc7
	.word	.Linfo_string39
	.byte	4
	.byte	233
	.word	883
	.byte	17
	.word	.Ldebug_loc8
	.word	.Linfo_string40
	.byte	4
	.byte	232
	.word	883
	.byte	20
	.word	.Linfo_string41
	.byte	4
	.byte	202
	.word	888
	.byte	16
	.quad	.Ltmp22
	.word	.Ltmp28-.Ltmp22
	.byte	17
	.word	.Ldebug_loc6
	.word	.Linfo_string26
	.byte	4
	.byte	222
	.word	64
	.byte	16
	.quad	.Ltmp23
	.word	.Ltmp25-.Ltmp23
	.byte	20
	.word	.Linfo_string42
	.byte	4
	.byte	223
	.word	810
	.byte	0
	.byte	0
	.byte	16
	.quad	.Ltmp31
	.word	.Ltmp38-.Ltmp31
	.byte	17
	.word	.Ldebug_loc9
	.word	.Linfo_string26
	.byte	4
	.byte	239
	.word	64
	.byte	0
	.byte	21
	.word	419
	.quad	.Ltmp30
	.byte	0
	.byte	22
	.quad	.Lfunc_begin4
	.word	.Lfunc_end4-.Lfunc_begin4
	.byte	1
	.byte	82

	.word	.Linfo_string20
	.byte	4
	.half	259
	.word	810

	.byte	23
	.word	.Ldebug_loc10
	.word	.Linfo_string43
	.byte	4
	.half	261
	.word	810
	.byte	24
	.byte	1
	.word	.Linfo_string44
	.byte	4
	.half	262
	.word	810
	.byte	21
	.word	528
	.quad	.Ltmp44
	.byte	0
	.byte	25
	.byte	2
	.word	.Linfo_string19
	.byte	5
	.byte	4
	.byte	3
	.word	822
	.byte	26
	.byte	27
	.word	161
	.byte	28
	.word	149
	.byte	4
	.byte	0
	.byte	27
	.word	847
	.byte	28
	.word	149
	.byte	128
	.byte	0
	.byte	10
	.byte	5
	.word	.Linfo_string37
	.byte	32
	.byte	4
	.byte	136
	.byte	16
	.byte	11
	.word	.Linfo_string35
	.word	189
	.byte	4
	.byte	138
	.byte	0
	.byte	29
	.word	.Linfo_string36
	.word	156
	.byte	4
	.byte	140
	.byte	16
	.byte	16
	.byte	0
	.byte	3
	.word	847
	.byte	27
	.word	810
	.byte	28
	.word	149
	.byte	128
	.byte	0
	.byte	0
.Ldebug_info_end0:
	.section	.debug_str,"MS",@progbits,1
.Linfo_string0:
	.asciz	"Ubuntu clang version 13.0.1-++20220120110924+75e33f71c2da-1~exp1~20220120231001.58"
.Linfo_string1:
	.asciz	"/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/vector_memcpy_pointers/vector_memcpy_pointers.cpp"
.Linfo_string2:
	.asciz	"/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/build/llvm-13-rv64imv/vector_memcpy_pointers"
.Linfo_string3:
	.asciz	"unsigned char"
.Linfo_string4:
	.asciz	"long unsigned int"
.Linfo_string5:
	.asciz	"size_t"
.Linfo_string6:
	.asciz	"uint8_t"
.Linfo_string7:
	.asciz	"__ARRAY_SIZE_TYPE__"
.Linfo_string8:
	.asciz	"__rvv_uint8m8_t"
.Linfo_string9:
	.asciz	"vuint8m8_t"
.Linfo_string10:
	.asciz	"value"
.Linfo_string11:
	.asciz	"uint64_t"
.Linfo_string12:
	.asciz	"Base"
.Linfo_string13:
	.asciz	"memset"
.Linfo_string14:
	.asciz	"memcpy"
.Linfo_string15:
	.asciz	"_Z13vector_memcpyPhPKhm"
.Linfo_string16:
	.asciz	"vector_memcpy"
.Linfo_string17:
	.asciz	"_Z13run_base_testv"
.Linfo_string18:
	.asciz	"run_base_test"
.Linfo_string19:
	.asciz	"int"
.Linfo_string20:
	.asciz	"main"
.Linfo_string21:
	.asciz	"dest"
.Linfo_string22:
	.asciz	"ch"
.Linfo_string23:
	.asciz	"count"
.Linfo_string24:
	.asciz	"ch_uc"
.Linfo_string25:
	.asciz	"dest_uc"
.Linfo_string26:
	.asciz	"i"
.Linfo_string27:
	.asciz	"src"
.Linfo_string28:
	.asciz	"src_uc"
.Linfo_string29:
	.asciz	"dst"
.Linfo_string30:
	.asciz	"num_bytes"
.Linfo_string31:
	.asciz	"copied_per_iter"
.Linfo_string32:
	.asciz	"data"
.Linfo_string33:
	.asciz	"bases"
.Linfo_string34:
	.asciz	"source_array"
.Linfo_string35:
	.asciz	"expected_base_value"
.Linfo_string36:
	.asciz	"base_ptr"
.Linfo_string37:
	.asciz	"Element"
.Linfo_string38:
	.asciz	"dest_array"
.Linfo_string39:
	.asciz	"dst_ptr"
.Linfo_string40:
	.asciz	"src_ptr"
.Linfo_string41:
	.asciz	"indices"
.Linfo_string42:
	.asciz	"index"
.Linfo_string43:
	.asciz	"result"
.Linfo_string44:
	.asciz	"attempted"
	.ident	"Ubuntu clang version 13.0.1-++20220120110924+75e33f71c2da-1~exp1~20220120231001.58"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym outputAttempted
	.addrsig_sym outputSucceeded
	.section	.debug_line,"",@progbits
.Lline_table_start0:
