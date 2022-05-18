	.text
	.attribute	4, 16
	.attribute	5, "rv32i2p0_m2p0_v0p10_zvlsseg0p10"
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

	.globl	_Z13vector_memcpyPhPKhj
	.p2align	2
	.type	_Z13vector_memcpyPhPKhj,@function
_Z13vector_memcpyPhPKhj:
.Lfunc_begin2:
	.loc	4 156 0
	.cfi_startproc
	.loc	4 175 5 prologue_end
	beqz	a2, .LBB2_2
.Ltmp13:
.LBB2_1:
	.loc	4 176 34
	vsetvli	a3, a2, e8, m8, ta, mu
.Ltmp14:
	.loc	4 183 16
	vle8.v	v8, (a1)
.Ltmp15:
	.loc	4 184 9
	vse8.v	v8, (a0)
	.loc	4 187 13
	add	a1, a1, a3
.Ltmp16:
	.loc	4 189 19
	sub	a2, a2, a3
.Ltmp17:
	.loc	4 188 13
	add	a0, a0, a3
.Ltmp18:
	.loc	4 175 5
	bnez	a2, .LBB2_1
.Ltmp19:
.LBB2_2:
	.loc	4 191 1
	ret
.Ltmp20:
.Lfunc_end2:
	.size	_Z13vector_memcpyPhPKhj, .Lfunc_end2-_Z13vector_memcpyPhPKhj
	.cfi_endproc

	.globl	_Z13run_base_testv
	.p2align	2
	.type	_Z13run_base_testv,@function
_Z13run_base_testv:
.Lfunc_begin3:
	.loc	4 239 0
	.cfi_startproc
	addi	sp, sp, -2032
	.cfi_def_cfa_offset 2032
.Ltmp21:
	.loc	4 243 10 prologue_end
	sw	ra, 2028(sp)
	sw	s0, 2024(sp)
	sw	s1, 2020(sp)
	sw	s2, 2016(sp)
	.cfi_offset ra, -4
	.cfi_offset s0, -8
	.cfi_offset s1, -12
	.cfi_offset s2, -16
	lui	a0, 2
	addi	a0, a0, -1936
	sub	sp, sp, a0
	.cfi_def_cfa_offset 8288
	lui	a0, 2
	addi	a0, a0, 16
	add	a0, sp, a0
	addi	a2, zero, 64
	lui	a1, 2
	addi	a1, a1, 16
	add	s2, sp, a1
	mv	a1, zero
	call	memset@plt
.Ltmp22:
	.loc	4 0 10 is_stmt 0
	mv	a0, zero
	lui	a1, 476911
	addi	a1, a1, 242
	.loc	4 243 10
	lui	a2, 2
	addi	a2, a2, 20
	add	a2, sp, a2
	sw	a1, 0(a2)
	lui	a1, 678729
	addi	a1, a1, 1882
	lui	a2, 2
	addi	a2, a2, 16
	add	a2, sp, a2
	sw	a1, 0(a2)
	lui	a1, 577150
	addi	a1, a1, 100
	lui	a2, 2
	addi	a2, a2, 36
	add	a2, sp, a2
	sw	a1, 0(a2)
	lui	a1, 239147
	addi	a1, a1, 1188
	lui	a2, 2
	addi	a2, a2, 32
	add	a2, sp, a2
	sw	a1, 0(a2)
	lui	a1, 422522
	addi	a1, a1, -1641
	lui	a2, 2
	addi	a2, a2, 52
	add	a2, sp, a2
	sw	a1, 0(a2)
	lui	a1, 115519
	addi	a1, a1, -802
	lui	a2, 2
	addi	a2, a2, 48
	add	a2, sp, a2
	sw	a1, 0(a2)
	lui	a1, 610294
	addi	a1, a1, -900
	lui	a2, 2
	addi	a2, a2, 68
	add	a2, sp, a2
	sw	a1, 0(a2)
	lui	a1, 482014
	addi	a1, a1, 1687
	lui	a2, 2
	addi	a2, a2, 64
	add	a2, sp, a2
	sw	a1, 0(a2)
.Ltmp23:
	.loc	4 272 5 is_stmt 1
	lui	a1, 1
	addi	a1, a1, 32
	add	a1, sp, a1
	lui	a2, %hi(.L__const._Z13run_base_testv.indices)
	addi	a2, a2, %lo(.L__const._Z13run_base_testv.indices)
	addi	a3, zero, 512
.Ltmp24:
.LBB3_1:
	.loc	4 273 21
	add	a4, a0, a2
	lw	a4, 0(a4)
.Ltmp25:
	.loc	4 275 36
	slli	a4, a4, 4
.Ltmp26:
	add	a4, s2, a4
	.loc	4 275 49 is_stmt 0
	ori	a5, a4, 4
	lw	s0, 0(a4)
	lw	a5, 0(a5)
	.loc	4 274 25 is_stmt 1
	sw	s0, -16(a1)
	sw	a5, -12(a1)
	lw	a5, 16(sp)
	lw	s0, 20(sp)
	lw	s1, 24(sp)
	sw	a4, 0(a1)
	sw	a5, 4(a1)
	sw	s0, 8(a1)
	sw	s1, 12(a1)
.Ltmp27:
	.loc	4 272 26
	addi	a0, a0, 4
.Ltmp28:
	addi	a1, a1, 32
.Ltmp29:
	.loc	4 272 5 is_stmt 0
	bne	a0, a3, .LBB3_1
.Ltmp30:
	.loc	4 280 13 is_stmt 1
	addi	a0, sp, 16
	lui	a2, 1
	mv	a1, zero
	call	memset@plt
.Ltmp31:
	.loc	4 286 5
	addi	a0, sp, 16
	lui	a1, 1
	addi	a1, a1, 16
	add	a1, sp, a1
	lui	a2, 1
	call	_Z13vector_memcpyPhPKhj
.Ltmp32:
	.loc	4 0 5 is_stmt 0
	mv	a0, zero
.Ltmp33:
	.loc	4 289 5 is_stmt 1
	addi	a1, sp, 32
	lui	a2, %hi(.L__const._Z13run_base_testv.indices)
	addi	a2, a2, %lo(.L__const._Z13run_base_testv.indices)
	lui	a3, 2
	addi	a3, a3, 16
	add	a7, sp, a3
	addi	a6, zero, 512
.Ltmp34:
.LBB3_3:
	.loc	4 292 27
	lw	a5, 0(a1)
	.loc	4 292 37 is_stmt 0
	lw	s1, 0(a5)
	lw	s0, 4(a5)
	.loc	4 292 60
	lw	a4, -12(a1)
	lw	a3, -16(a1)
	.loc	4 292 43
	xor	a4, s0, a4
	xor	a3, s1, a3
	or	a3, a3, a4
	bnez	a3, .LBB3_7
.Ltmp35:
	.loc	4 295 78 is_stmt 1
	add	a3, a0, a2
	lw	a3, 0(a3)
	.loc	4 295 72 is_stmt 0
	slli	a3, a3, 4
	add	a3, a7, a3
.Ltmp36:
	.loc	4 0 72
	bne	a5, a3, .LBB3_7
.Ltmp37:
	.loc	4 289 26 is_stmt 1
	addi	a0, a0, 4
.Ltmp38:
	addi	a1, a1, 32
.Ltmp39:
	.loc	4 289 5 is_stmt 0
	bne	a0, a6, .LBB3_3
.Ltmp40:
	.loc	4 0 5
	addi	a0, zero, 1
	j	.LBB3_8
.Ltmp41:
.LBB3_7:
	mv	a0, zero
.Ltmp42:
.LBB3_8:
	.loc	4 300 1 is_stmt 1
	lui	a1, 2
	addi	a1, a1, -1936
	add	sp, sp, a1
	lw	s2, 2016(sp)
	lw	s1, 2020(sp)
	lw	s0, 2024(sp)
	lw	ra, 2028(sp)
	addi	sp, sp, 2032
	ret
.Ltmp43:
.Lfunc_end3:
	.size	_Z13run_base_testv, .Lfunc_end3-_Z13run_base_testv
	.cfi_endproc

	.globl	main
	.p2align	2
	.type	main,@function
main:
.Lfunc_begin4:
	.loc	4 374 0
	.cfi_startproc
	addi	sp, sp, -16
	.cfi_def_cfa_offset 16
.Ltmp44:
	.loc	4 0 0 prologue_end
	sw	ra, 12(sp)
	.cfi_offset ra, -4
.Ltmp45:
	.loc	4 379 15
	call	_Z13run_base_testv
.Ltmp46:
	.loc	4 0 15 is_stmt 0
	lui	a1, %hi(outputAttempted)
.Ltmp47:
	.loc	4 386 25 is_stmt 1
	sw	zero, %lo(outputAttempted+4)(a1)
	addi	a2, zero, 1
	sw	a2, %lo(outputAttempted)(a1)
	lui	a1, %hi(outputSucceeded)
	.loc	4 387 25
	sw	a0, %lo(outputSucceeded)(a1)
	sw	zero, %lo(outputSucceeded+4)(a1)
	.loc	4 388 14
	lui	a1, %hi(finished)
	sb	a2, %lo(finished)(a1)
	.loc	4 389 5
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Ltmp48:
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
	.word	.Lfunc_begin0-.Lfunc_begin0
	.word	.Ltmp1-.Lfunc_begin0
	.half	1
	.byte	92
	.word	0
	.word	0
.Ldebug_loc1:
	.word	.Lfunc_begin0-.Lfunc_begin0
	.word	.Ltmp1-.Lfunc_begin0
	.half	3
	.byte	17
	.byte	0
	.byte	159
	.word	0
	.word	0
.Ldebug_loc2:
	.word	.Lfunc_begin1-.Lfunc_begin0
	.word	.Ltmp7-.Lfunc_begin0
	.half	1
	.byte	91
	.word	0
	.word	0
.Ldebug_loc3:
	.word	.Lfunc_begin1-.Lfunc_begin0
	.word	.Ltmp7-.Lfunc_begin0
	.half	1
	.byte	90
	.word	.Ltmp7-.Lfunc_begin0
	.word	.Ltmp11-.Lfunc_begin0
	.half	1
	.byte	93
	.word	0
	.word	0
.Ldebug_loc4:
	.word	.Ltmp14-.Lfunc_begin0
	.word	.Ltmp19-.Lfunc_begin0
	.half	1
	.byte	93
	.word	0
	.word	0
.Ldebug_loc5:
	.word	.Ltmp15-.Lfunc_begin0
	.word	.Ltmp19-.Lfunc_begin0
	.half	2
	.byte	144
	.byte	104
	.word	0
	.word	0
.Ldebug_loc6:
	.word	.Ltmp23-.Lfunc_begin0
	.word	.Ltmp24-.Lfunc_begin0
	.half	2
	.byte	48
	.byte	159
	.word	.Ltmp24-.Lfunc_begin0
	.word	.Ltmp27-.Lfunc_begin0
	.half	6
	.byte	122
	.byte	0
	.byte	17
	.byte	4
	.byte	27
	.byte	159
	.word	.Ltmp27-.Lfunc_begin0
	.word	.Ltmp28-.Lfunc_begin0
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
	.word	0
	.word	0
.Ldebug_loc7:
	.word	.Ltmp25-.Lfunc_begin0
	.word	.Ltmp26-.Lfunc_begin0
	.half	1
	.byte	94
	.word	0
	.word	0
.Ldebug_loc8:
	.word	.Ltmp31-.Lfunc_begin0
	.word	.Lfunc_end3-.Lfunc_begin0
	.half	3
	.byte	114
	.byte	16
	.byte	159
	.word	0
	.word	0
.Ldebug_loc9:
	.word	.Ltmp31-.Lfunc_begin0
	.word	.Lfunc_end3-.Lfunc_begin0
	.half	4
	.byte	114
	.byte	144
	.byte	32
	.byte	159
	.word	0
	.word	0
.Ldebug_loc10:
	.word	.Ltmp32-.Lfunc_begin0
	.word	.Ltmp34-.Lfunc_begin0
	.half	2
	.byte	48
	.byte	159
	.word	.Ltmp34-.Lfunc_begin0
	.word	.Ltmp36-.Lfunc_begin0
	.half	6
	.byte	122
	.byte	0
	.byte	17
	.byte	4
	.byte	27
	.byte	159
	.word	.Ltmp36-.Lfunc_begin0
	.word	.Ltmp38-.Lfunc_begin0
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
	.word	0
	.word	0
.Ldebug_loc11:
	.word	.Ltmp45-.Lfunc_begin0
	.word	.Ltmp46-.Lfunc_begin0
	.half	2
	.byte	48
	.byte	159
	.word	.Ltmp46-.Lfunc_begin0
	.word	.Ltmp47-.Lfunc_begin0
	.half	3
	.byte	90
	.byte	147
	.byte	4
	.word	.Ltmp47-.Lfunc_begin0
	.word	.Lfunc_end4-.Lfunc_begin0
	.half	6
	.byte	90
	.byte	147
	.byte	4
	.byte	80
	.byte	147
	.byte	4
	.word	0
	.word	0
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
	.byte	2
	.byte	24
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
	.byte	21
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
	.byte	22
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
	.byte	23
	.ascii	"\211\202\001"
	.byte	0
	.byte	49
	.byte	19
	.byte	17
	.byte	1
	.byte	0
	.byte	0
	.byte	24
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
	.byte	25
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
	.byte	26
	.byte	15
	.byte	0
	.byte	0
	.byte	0
	.byte	27
	.byte	38
	.byte	0
	.byte	0
	.byte	0
	.byte	28
	.byte	1
	.byte	1
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	29
	.byte	33
	.byte	0
	.byte	73
	.byte	19
	.byte	55
	.byte	11
	.byte	0
	.byte	0
	.byte	30
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
	.byte	4
	.byte	1
	.word	.Linfo_string0
	.half	33
	.word	.Linfo_string1
	.word	.Lline_table_start0
	.word	.Linfo_string2
	.word	.Lfunc_begin0
	.word	.Lfunc_end4-.Lfunc_begin0
	.byte	2
	.word	.Linfo_string3
	.byte	8
	.byte	1
	.byte	3
	.word	38
	.byte	3
	.word	55
	.byte	4
	.word	38
	.byte	5
	.word	71
	.word	.Linfo_string5
	.byte	1
	.byte	46
	.byte	2
	.word	.Linfo_string4
	.byte	7
	.byte	4
	.byte	3
	.word	83
	.byte	4
	.word	88
	.byte	5
	.word	38
	.word	.Linfo_string6
	.byte	2
	.byte	226
	.byte	3
	.word	88
	.byte	5
	.word	115
	.word	.Linfo_string9
	.byte	3
	.byte	99
	.byte	6
	.word	124
	.word	.Linfo_string8
	.byte	7

	.word	38
	.byte	8
	.word	145
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
	.word	157
	.byte	10
	.byte	5
	.word	.Linfo_string13
	.byte	16
	.byte	4
	.byte	134
	.byte	16
	.byte	11
	.word	.Linfo_string10
	.word	180
	.byte	4
	.byte	135
	.byte	0
	.byte	0
	.byte	5
	.word	191
	.word	.Linfo_string12
	.byte	2
	.byte	98
	.byte	2
	.word	.Linfo_string11
	.byte	7
	.byte	8
	.byte	12
	.word	.Lfunc_begin0
	.word	.Lfunc_end0-.Lfunc_begin0
	.byte	1
	.byte	82

	.word	.Linfo_string14
	.byte	4
	.byte	8
	.word	770

	.byte	13
	.byte	1
	.byte	90
	.word	.Linfo_string22
	.byte	4
	.byte	8
	.word	770
	.byte	13
	.byte	1
	.byte	91
	.word	.Linfo_string23
	.byte	4
	.byte	8
	.word	771
	.byte	14
	.word	.Ldebug_loc0
	.word	.Linfo_string24
	.byte	4
	.byte	8
	.word	60
	.byte	15
	.byte	3
	.byte	123
	.byte	0
	.byte	159
	.word	.Linfo_string25
	.byte	4
	.byte	9
	.word	38
	.byte	15
	.byte	1
	.byte	90
	.word	.Linfo_string26
	.byte	4
	.byte	10
	.word	45
	.byte	16
	.word	.Lfunc_begin0
	.word	.Ltmp4-.Lfunc_begin0
	.byte	17
	.word	.Ldebug_loc1
	.word	.Linfo_string27
	.byte	4
	.byte	11
	.word	771
	.byte	0
	.byte	0
	.byte	12
	.word	.Lfunc_begin1
	.word	.Lfunc_end1-.Lfunc_begin1
	.byte	1
	.byte	82

	.word	.Linfo_string15
	.byte	4
	.byte	19
	.word	770

	.byte	13
	.byte	1
	.byte	90
	.word	.Linfo_string22
	.byte	4
	.byte	19
	.word	770
	.byte	14
	.word	.Ldebug_loc2
	.word	.Linfo_string28
	.byte	4
	.byte	19
	.word	778
	.byte	13
	.byte	1
	.byte	92
	.word	.Linfo_string24
	.byte	4
	.byte	19
	.word	60
	.byte	17
	.word	.Ldebug_loc3
	.word	.Linfo_string26
	.byte	4
	.byte	20
	.word	45
	.byte	15
	.byte	1
	.byte	91
	.word	.Linfo_string29
	.byte	4
	.byte	21
	.word	50
	.byte	0
	.byte	18
	.word	.Lfunc_begin2
	.word	.Lfunc_end2-.Lfunc_begin2
	.byte	1
	.byte	82

	.word	.Linfo_string16
	.word	.Linfo_string17
	.byte	4
	.byte	156

	.byte	13
	.byte	1
	.byte	90
	.word	.Linfo_string30
	.byte	4
	.byte	156
	.word	99
	.byte	13
	.byte	1
	.byte	91
	.word	.Linfo_string28
	.byte	4
	.byte	156
	.word	78
	.byte	13
	.byte	1
	.byte	92
	.word	.Linfo_string31
	.byte	4
	.byte	156
	.word	60
	.byte	16
	.word	.Ltmp13
	.word	.Ltmp18-.Ltmp13
	.byte	17
	.word	.Ldebug_loc4
	.word	.Linfo_string32
	.byte	4
	.byte	176
	.word	60
	.byte	17
	.word	.Ldebug_loc5
	.word	.Linfo_string33
	.byte	4
	.byte	178
	.word	104
	.byte	0
	.byte	0
	.byte	19
	.word	.Lfunc_begin3
	.word	.Lfunc_end3-.Lfunc_begin3
	.byte	1
	.byte	82

	.word	.Linfo_string18
	.word	.Linfo_string19
	.byte	4
	.byte	239
	.word	771

	.byte	15
	.byte	4
	.byte	145
	.asciz	"\220\300"
	.word	.Linfo_string34
	.byte	4
	.byte	243
	.word	784
	.byte	20
	.byte	3
	.byte	145
	.ascii	"\220 "
	.word	.Linfo_string35
	.byte	4
	.half	271
	.word	796
	.byte	20
	.byte	2
	.byte	145
	.byte	16
	.word	.Linfo_string39
	.byte	4
	.half	280
	.word	796
	.byte	21
	.word	.Ldebug_loc8
	.word	.Linfo_string41
	.byte	4
	.half	283
	.word	844
	.byte	21
	.word	.Ldebug_loc9
	.word	.Linfo_string42
	.byte	4
	.half	282
	.word	844
	.byte	22
	.word	.Linfo_string43
	.byte	4
	.byte	252
	.word	849
	.byte	16
	.word	.Ltmp23
	.word	.Ltmp30-.Ltmp23
	.byte	21
	.word	.Ldebug_loc6
	.word	.Linfo_string27
	.byte	4
	.half	272
	.word	60
	.byte	16
	.word	.Ltmp24
	.word	.Ltmp27-.Ltmp24
	.byte	21
	.word	.Ldebug_loc7
	.word	.Linfo_string40
	.byte	4
	.half	273
	.word	771
	.byte	0
	.byte	0
	.byte	16
	.word	.Ltmp33
	.word	.Ltmp40-.Ltmp33
	.byte	21
	.word	.Ldebug_loc10
	.word	.Linfo_string27
	.byte	4
	.half	289
	.word	60
	.byte	0
	.byte	23
	.word	405
	.word	.Ltmp32
	.byte	0
	.byte	24
	.word	.Lfunc_begin4
	.word	.Lfunc_end4-.Lfunc_begin4
	.byte	1
	.byte	82

	.word	.Linfo_string21
	.byte	4
	.half	373
	.word	771

	.byte	21
	.word	.Ldebug_loc11
	.word	.Linfo_string44
	.byte	4
	.half	375
	.word	861
	.byte	25
	.byte	1
	.word	.Linfo_string47
	.byte	4
	.half	376
	.word	861
	.byte	23
	.word	506
	.word	.Ltmp46
	.byte	0
	.byte	26
	.byte	2
	.word	.Linfo_string20
	.byte	5
	.byte	4
	.byte	3
	.word	783
	.byte	27
	.byte	28
	.word	157
	.byte	29
	.word	145
	.byte	4
	.byte	0
	.byte	28
	.word	808
	.byte	29
	.word	145
	.byte	128
	.byte	0
	.byte	10
	.byte	5
	.word	.Linfo_string38
	.byte	32
	.byte	4
	.byte	139
	.byte	16
	.byte	11
	.word	.Linfo_string36
	.word	180
	.byte	4
	.byte	141
	.byte	0
	.byte	30
	.word	.Linfo_string37
	.word	152
	.byte	4
	.byte	143
	.byte	16
	.byte	16
	.byte	0
	.byte	3
	.word	808
	.byte	28
	.word	771
	.byte	29
	.word	145
	.byte	128
	.byte	0
	.byte	5
	.word	872
	.word	.Linfo_string46
	.byte	2
	.byte	96
	.byte	2
	.word	.Linfo_string45
	.byte	5
	.byte	8
	.byte	0
.Ldebug_info_end0:
	.section	.debug_str,"MS",@progbits,1
.Linfo_string0:
	.asciz	"Ubuntu clang version 13.0.1-++20220120110924+75e33f71c2da-1~exp1~20220120231001.58"
.Linfo_string1:
	.asciz	"/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/vector_memcpy_pointers/vector_memcpy_pointers.cpp"
.Linfo_string2:
	.asciz	"/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/build/llvm-13-rv32imv/vector_memcpy_pointers"
.Linfo_string3:
	.asciz	"unsigned char"
.Linfo_string4:
	.asciz	"unsigned int"
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
	.asciz	"long long unsigned int"
.Linfo_string12:
	.asciz	"uint64_t"
.Linfo_string13:
	.asciz	"Base"
.Linfo_string14:
	.asciz	"memset"
.Linfo_string15:
	.asciz	"memcpy"
.Linfo_string16:
	.asciz	"_Z13vector_memcpyPhPKhj"
.Linfo_string17:
	.asciz	"vector_memcpy"
.Linfo_string18:
	.asciz	"_Z13run_base_testv"
.Linfo_string19:
	.asciz	"run_base_test"
.Linfo_string20:
	.asciz	"int"
.Linfo_string21:
	.asciz	"main"
.Linfo_string22:
	.asciz	"dest"
.Linfo_string23:
	.asciz	"ch"
.Linfo_string24:
	.asciz	"count"
.Linfo_string25:
	.asciz	"ch_uc"
.Linfo_string26:
	.asciz	"dest_uc"
.Linfo_string27:
	.asciz	"i"
.Linfo_string28:
	.asciz	"src"
.Linfo_string29:
	.asciz	"src_uc"
.Linfo_string30:
	.asciz	"dst"
.Linfo_string31:
	.asciz	"num_bytes"
.Linfo_string32:
	.asciz	"copied_per_iter"
.Linfo_string33:
	.asciz	"data"
.Linfo_string34:
	.asciz	"bases"
.Linfo_string35:
	.asciz	"source_array"
.Linfo_string36:
	.asciz	"expected_base_value"
.Linfo_string37:
	.asciz	"base_ptr"
.Linfo_string38:
	.asciz	"Element"
.Linfo_string39:
	.asciz	"dest_array"
.Linfo_string40:
	.asciz	"index"
.Linfo_string41:
	.asciz	"dst_ptr"
.Linfo_string42:
	.asciz	"src_ptr"
.Linfo_string43:
	.asciz	"indices"
.Linfo_string44:
	.asciz	"result"
.Linfo_string45:
	.asciz	"long long int"
.Linfo_string46:
	.asciz	"int64_t"
.Linfo_string47:
	.asciz	"attempted"
	.ident	"Ubuntu clang version 13.0.1-++20220120110924+75e33f71c2da-1~exp1~20220120231001.58"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym outputAttempted
	.addrsig_sym outputSucceeded
	.addrsig_sym finished
	.section	.debug_line,"",@progbits
.Lline_table_start0:
