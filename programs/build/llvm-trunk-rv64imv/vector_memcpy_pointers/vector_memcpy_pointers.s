	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p0_m2p0_f2p0_d2p0_v1p0_zvl128b1p0_zvl32b1p0_zvl64b1p0"
	.file	"vector_memcpy_pointers.cpp"
	.file	0 "/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/build/llvm-trunk-rv64imv/vector_memcpy_pointers" "/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/vector_memcpy_pointers/vector_memcpy_pointers.cpp" md5 0x1ff4a29792c57ebf3560855a0770f1b2
	.file	1 "/home/samuel/repos/llvm-project/build/lib/clang/15.0.0/include" "stddef.h" md5 0x2499dd2361b915724b073282bea3a7bc
	.file	2 "/home/samuel/repos/llvm-project/build/lib/clang/15.0.0/include" "stdint.h" md5 0x65478c86519fc5f031066ff00d1e57d5
	.file	3 "/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs" "vector_memcpy_pointers/vector_memcpy_pointers.cpp" md5 0x1ff4a29792c57ebf3560855a0770f1b2
	.globl	memset
	.p2align	2
	.type	memset,@function
memset:
.Lfunc_begin0:
	.loc	3 8 0
	.cfi_sections .debug_frame
	.cfi_startproc
	.loc	3 11 5 prologue_end
	beqz	a2, .LBB0_3
.Ltmp0:
	.loc	3 0 5 is_stmt 0
	mv	a3, a0
.Ltmp1:
.LBB0_2:
	.loc	3 12 24 is_stmt 1
	sb	a1, 0(a3)
.Ltmp2:
	.loc	3 11 23
	addi	a2, a2, -1
	addi	a3, a3, 1
.Ltmp3:
	.loc	3 11 5 is_stmt 0
	bnez	a2, .LBB0_2
.Ltmp4:
.LBB0_3:
	.loc	3 15 5 is_stmt 1
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
	.loc	3 19 0
	.cfi_startproc
	.loc	3 22 5 prologue_end
	beqz	a2, .LBB1_3
.Ltmp6:
	.loc	3 0 5 is_stmt 0
	mv	a3, a0
.Ltmp7:
.LBB1_2:
	.loc	3 23 20 is_stmt 1
	lb	a4, 0(a1)
	.loc	3 23 18 is_stmt 0
	sb	a4, 0(a3)
	.loc	3 24 16 is_stmt 1
	addi	a3, a3, 1
.Ltmp8:
	.loc	3 26 14
	addi	a2, a2, -1
.Ltmp9:
	.loc	3 25 15
	addi	a1, a1, 1
.Ltmp10:
	.loc	3 22 5
	bnez	a2, .LBB1_2
.Ltmp11:
.LBB1_3:
	.loc	3 28 5
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
	.loc	3 153 0
	.cfi_startproc
	.loc	3 172 5 prologue_end
	beqz	a2, .LBB2_2
.Ltmp13:
.LBB2_1:
	.loc	3 173 34
	vsetvli	a3, a2, e8, m8, ta, mu
.Ltmp14:
	.loc	3 180 16
	vle8.v	v8, (a1)
.Ltmp15:
	.loc	3 181 9
	vse8.v	v8, (a0)
	.loc	3 184 13
	add	a1, a1, a3
.Ltmp16:
	.loc	3 186 19
	sub	a2, a2, a3
.Ltmp17:
	.loc	3 185 13
	add	a0, a0, a3
.Ltmp18:
	.loc	3 172 5
	bnez	a2, .LBB2_1
.Ltmp19:
.LBB2_2:
	.loc	3 188 1
	ret
.Ltmp20:
.Lfunc_end2:
	.size	_Z13vector_memcpyPhPKhm, .Lfunc_end2-_Z13vector_memcpyPhPKhm
	.cfi_endproc

	.section	.sdata,"aw",@progbits
	.p2align	3
.LCPI3_0:
	.quad	8389908080790640474
.LCPI3_1:
	.quad	-8293413467698580316
.LCPI3_2:
	.quad	7433078584290569438
.LCPI3_3:
	.quad	-7710342347414968681
	.text
	.globl	_Z13run_base_testv
	.p2align	2
	.type	_Z13run_base_testv,@function
_Z13run_base_testv:
.Lfunc_begin3:
	.loc	3 243 0
	.cfi_startproc
	addi	sp, sp, -2032
	.cfi_def_cfa_offset 2032
	sd	ra, 2024(sp)
	.cfi_offset ra, -8
	lui	a0, 2
	addiw	a0, a0, -1936
	sub	sp, sp, a0
	.cfi_def_cfa_offset 8288
	li	a0, 0
.Ltmp21:
	.loc	3 247 10 prologue_end
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
	lui	a1, %hi(.LCPI3_0)
	ld	a1, %lo(.LCPI3_0)(a1)
	lui	a2, %hi(.LCPI3_1)
	ld	a2, %lo(.LCPI3_1)(a2)
	lui	a3, %hi(.LCPI3_2)
	ld	a3, %lo(.LCPI3_2)(a3)
	lui	a4, %hi(.LCPI3_3)
	ld	a4, %lo(.LCPI3_3)(a4)
	lui	a5, 2
	addiw	a5, a5, 16
	add	a5, sp, a5
	sd	a1, 0(a5)
	lui	a1, 2
	addiw	a1, a1, 32
	add	a1, sp, a1
	sd	a2, 0(a1)
	lui	a1, 2
	addiw	a1, a1, 48
	add	a1, sp, a1
	sd	a3, 0(a1)
	lui	a1, 2
	addiw	a1, a1, 64
	add	a1, sp, a1
	sd	a4, 0(a1)
.Ltmp22:
	.loc	3 276 5
	lui	a1, 1
	addiw	a1, a1, 32
	add	a1, sp, a1
	lui	a2, %hi(.L__const._Z13run_base_testv.indices)
	addi	a2, a2, %lo(.L__const._Z13run_base_testv.indices)
	lui	a3, 2
	addiw	a3, a3, 16
	add	a3, sp, a3
	li	a4, 512
.Ltmp23:
.LBB3_1:
	.loc	3 277 21
	add	a5, a0, a2
	lw	a5, 0(a5)
.Ltmp24:
	.loc	3 279 36
	slli	a5, a5, 4
	add	a5, a3, a5
	.loc	3 279 49 is_stmt 0
	ld	a6, 0(a5)
	.loc	3 278 25 is_stmt 1
	sd	a6, -16(a1)
	sd	a5, 0(a1)
.Ltmp25:
	.loc	3 276 26
	addi	a0, a0, 4
.Ltmp26:
	addi	a1, a1, 32
.Ltmp27:
	.loc	3 276 5 is_stmt 0
	bne	a0, a4, .LBB3_1
.Ltmp28:
	.loc	3 284 13 is_stmt 1
	addi	a0, sp, 16
	lui	a2, 1
	li	a1, 0
	call	memset@plt
.Ltmp29:
	.loc	3 290 5
	addi	a0, sp, 16
	lui	a1, 1
	addiw	a1, a1, 16
	add	a1, sp, a1
	lui	a2, 1
	call	_Z13vector_memcpyPhPKhm
.Ltmp30:
	.loc	3 0 5 is_stmt 0
	li	a0, 0
.Ltmp31:
	.loc	3 293 5 is_stmt 1
	addi	a1, sp, 32
	lui	a2, %hi(.L__const._Z13run_base_testv.indices)
	addi	a2, a2, %lo(.L__const._Z13run_base_testv.indices)
	lui	a3, 2
	addiw	a3, a3, 16
	add	a3, sp, a3
	li	a4, 512
.Ltmp32:
.LBB3_3:
	.loc	3 296 27
	ld	a5, 0(a1)
	.loc	3 296 37 is_stmt 0
	ld	a6, 0(a5)
	.loc	3 296 60
	ld	a7, -16(a1)
	bne	a6, a7, .LBB3_7
.Ltmp33:
	.loc	3 299 78 is_stmt 1
	add	a6, a0, a2
	lw	a6, 0(a6)
	.loc	3 299 72 is_stmt 0
	slli	a6, a6, 4
	add	a6, a3, a6
.Ltmp34:
	.loc	3 0 72
	bne	a5, a6, .LBB3_7
.Ltmp35:
	.loc	3 293 26 is_stmt 1
	addi	a0, a0, 4
.Ltmp36:
	addi	a1, a1, 32
.Ltmp37:
	.loc	3 293 5 is_stmt 0
	bne	a0, a4, .LBB3_3
.Ltmp38:
	.loc	3 0 5
	li	a0, 1
	j	.LBB3_8
.Ltmp39:
.LBB3_7:
	li	a0, 0
.Ltmp40:
.LBB3_8:
	.loc	3 304 1 is_stmt 1
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
	.loc	3 377 0
	.cfi_startproc
	addi	sp, sp, -16
	.cfi_def_cfa_offset 16
.Ltmp42:
	.loc	3 0 0 is_stmt 0
	sd	ra, 8(sp)
	.cfi_offset ra, -8
.Ltmp43:
	.loc	3 382 15 prologue_end is_stmt 1
	call	_Z13run_base_testv
.Ltmp44:
	.loc	3 389 25
	lui	a1, %hi(outputAttempted)
	li	a2, 1
	sw	a2, %lo(outputAttempted)(a1)
	.loc	3 390 25
	lui	a1, %hi(outputSucceeded)
	sw	a0, %lo(outputSucceeded)(a1)
	.loc	3 391 5
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

	.file	4 "/home/samuel/repos/llvm-project/build/lib/clang/15.0.0/include" "riscv_vector.h" md5 0x562f153fc5d087e86f62d5a02cfa94b1
	.section	.debug_loclists,"",@progbits
	.word	.Ldebug_list_header_end0-.Ldebug_list_header_start0
.Ldebug_list_header_start0:
	.half	5
	.byte	8
	.byte	0
	.word	11
.Lloclists_table_base0:
	.word	.Ldebug_loc0-.Lloclists_table_base0
	.word	.Ldebug_loc1-.Lloclists_table_base0
	.word	.Ldebug_loc2-.Lloclists_table_base0
	.word	.Ldebug_loc3-.Lloclists_table_base0
	.word	.Ldebug_loc4-.Lloclists_table_base0
	.word	.Ldebug_loc5-.Lloclists_table_base0
	.word	.Ldebug_loc6-.Lloclists_table_base0
	.word	.Ldebug_loc7-.Lloclists_table_base0
	.word	.Ldebug_loc8-.Lloclists_table_base0
	.word	.Ldebug_loc9-.Lloclists_table_base0
	.word	.Ldebug_loc10-.Lloclists_table_base0
.Ldebug_loc0:
	.byte	4
	.uleb128 .Lfunc_begin0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Lfunc_begin0
	.byte	1
	.byte	92
	.byte	0
.Ldebug_loc1:
	.byte	4
	.uleb128 .Lfunc_begin0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Lfunc_begin0
	.byte	3
	.byte	17
	.byte	0
	.byte	159
	.byte	4
	.uleb128 .Ltmp1-.Lfunc_begin0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.byte	6
	.byte	125
	.byte	0
	.byte	122
	.byte	0
	.byte	28
	.byte	159
	.byte	4
	.uleb128 .Ltmp2-.Lfunc_begin0
	.uleb128 .Ltmp3-.Lfunc_begin0
	.byte	9
	.byte	125
	.byte	0
	.byte	122
	.byte	0
	.byte	28
	.byte	17
	.byte	1
	.byte	34
	.byte	159
	.byte	0
.Ldebug_loc2:
	.byte	4
	.uleb128 .Lfunc_begin1-.Lfunc_begin0
	.uleb128 .Ltmp7-.Lfunc_begin0
	.byte	1
	.byte	91
	.byte	0
.Ldebug_loc3:
	.byte	4
	.uleb128 .Lfunc_begin1-.Lfunc_begin0
	.uleb128 .Ltmp7-.Lfunc_begin0
	.byte	1
	.byte	90
	.byte	4
	.uleb128 .Ltmp7-.Lfunc_begin0
	.uleb128 .Ltmp11-.Lfunc_begin0
	.byte	1
	.byte	93
	.byte	0
.Ldebug_loc4:
	.byte	4
	.uleb128 .Ltmp14-.Lfunc_begin0
	.uleb128 .Ltmp19-.Lfunc_begin0
	.byte	1
	.byte	93
	.byte	0
.Ldebug_loc5:
	.byte	4
	.uleb128 .Ltmp15-.Lfunc_begin0
	.uleb128 .Ltmp19-.Lfunc_begin0
	.byte	2
	.byte	144
	.byte	104
	.byte	0
.Ldebug_loc6:
	.byte	4
	.uleb128 .Ltmp22-.Lfunc_begin0
	.uleb128 .Ltmp23-.Lfunc_begin0
	.byte	2
	.byte	48
	.byte	159
	.byte	4
	.uleb128 .Ltmp23-.Lfunc_begin0
	.uleb128 .Ltmp25-.Lfunc_begin0
	.byte	6
	.byte	122
	.byte	0
	.byte	17
	.byte	4
	.byte	27
	.byte	159
	.byte	4
	.uleb128 .Ltmp25-.Lfunc_begin0
	.uleb128 .Ltmp26-.Lfunc_begin0
	.byte	9
	.byte	122
	.byte	0
	.byte	17
	.byte	4
	.byte	27
	.byte	17
	.byte	1
	.byte	34
	.byte	159
	.byte	0
.Ldebug_loc7:
	.byte	4
	.uleb128 .Ltmp29-.Lfunc_begin0
	.uleb128 .Lfunc_end3-.Lfunc_begin0
	.byte	3
	.byte	114
	.byte	16
	.byte	159
	.byte	0
.Ldebug_loc8:
	.byte	4
	.uleb128 .Ltmp29-.Lfunc_begin0
	.uleb128 .Lfunc_end3-.Lfunc_begin0
	.byte	4
	.byte	114
	.byte	144
	.byte	32
	.byte	159
	.byte	0
.Ldebug_loc9:
	.byte	4
	.uleb128 .Ltmp30-.Lfunc_begin0
	.uleb128 .Ltmp32-.Lfunc_begin0
	.byte	2
	.byte	48
	.byte	159
	.byte	4
	.uleb128 .Ltmp32-.Lfunc_begin0
	.uleb128 .Ltmp34-.Lfunc_begin0
	.byte	6
	.byte	122
	.byte	0
	.byte	17
	.byte	4
	.byte	27
	.byte	159
	.byte	4
	.uleb128 .Ltmp34-.Lfunc_begin0
	.uleb128 .Ltmp36-.Lfunc_begin0
	.byte	9
	.byte	122
	.byte	0
	.byte	17
	.byte	4
	.byte	27
	.byte	17
	.byte	1
	.byte	34
	.byte	159
	.byte	0
.Ldebug_loc10:
	.byte	4
	.uleb128 .Ltmp43-.Lfunc_begin0
	.uleb128 .Ltmp44-.Lfunc_begin0
	.byte	3
	.byte	17
	.byte	0
	.byte	159
	.byte	0
.Ldebug_list_header_end0:
	.section	.debug_abbrev,"",@progbits
	.byte	1
	.byte	17
	.byte	1
	.byte	37
	.byte	37
	.byte	19
	.byte	5
	.byte	3
	.byte	37
	.byte	114
	.byte	23
	.byte	16
	.byte	23
	.byte	27
	.byte	37
	.byte	17
	.byte	27
	.byte	18
	.byte	6
	.byte	115
	.byte	23
	.ascii	"\214\001"
	.byte	23
	.byte	0
	.byte	0
	.byte	2
	.byte	36
	.byte	0
	.byte	3
	.byte	37
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
	.byte	37
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	0
	.byte	0
	.byte	6
	.byte	19
	.byte	1
	.byte	54
	.byte	11
	.byte	3
	.byte	37
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
	.byte	7
	.byte	13
	.byte	0
	.byte	3
	.byte	37
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
	.byte	8
	.byte	46
	.byte	1
	.byte	17
	.byte	27
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	122
	.byte	25
	.byte	3
	.byte	37
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
	.byte	9
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	37
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	10
	.byte	5
	.byte	0
	.byte	2
	.byte	34
	.byte	3
	.byte	37
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	11
	.byte	52
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	37
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	12
	.byte	11
	.byte	1
	.byte	17
	.byte	27
	.byte	18
	.byte	6
	.byte	0
	.byte	0
	.byte	13
	.byte	52
	.byte	0
	.byte	2
	.byte	34
	.byte	3
	.byte	37
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	14
	.byte	46
	.byte	1
	.byte	17
	.byte	27
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	122
	.byte	25
	.byte	110
	.byte	37
	.byte	3
	.byte	37
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	15
	.byte	46
	.byte	1
	.byte	17
	.byte	27
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	122
	.byte	25
	.byte	110
	.byte	37
	.byte	3
	.byte	37
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
	.byte	16
	.byte	52
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	37
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	17
	.byte	52
	.byte	0
	.byte	2
	.byte	34
	.byte	3
	.byte	37
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	18
	.byte	52
	.byte	0
	.byte	3
	.byte	37
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	19
	.byte	72
	.byte	0
	.byte	127
	.byte	19
	.byte	125
	.byte	27
	.byte	0
	.byte	0
	.byte	20
	.byte	46
	.byte	1
	.byte	17
	.byte	27
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	122
	.byte	25
	.byte	3
	.byte	37
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
	.byte	21
	.byte	52
	.byte	0
	.byte	28
	.byte	13
	.byte	3
	.byte	37
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	22
	.byte	15
	.byte	0
	.byte	0
	.byte	0
	.byte	23
	.byte	38
	.byte	0
	.byte	0
	.byte	0
	.byte	24
	.byte	22
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	37
	.byte	0
	.byte	0
	.byte	25
	.byte	1
	.byte	1
	.ascii	"\207B"
	.byte	25
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	26
	.byte	33
	.byte	0
	.byte	73
	.byte	19
	.byte	47
	.byte	24
	.byte	0
	.byte	0
	.byte	27
	.byte	36
	.byte	0
	.byte	3
	.byte	37
	.byte	11
	.byte	11
	.byte	62
	.byte	11
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
	.byte	37
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
	.half	5
	.byte	1
	.byte	8
	.word	.debug_abbrev
	.byte	1
	.byte	0
	.half	33
	.byte	1
	.word	.Lstr_offsets_base0
	.word	.Lline_table_start0
	.byte	2
	.byte	0
	.word	.Lfunc_end4-.Lfunc_begin0
	.word	.Laddr_table_base0
	.word	.Lloclists_table_base0
	.byte	2
	.byte	11
	.byte	7
	.byte	4
	.byte	2
	.byte	10
	.byte	7
	.byte	1
	.byte	2
	.byte	3
	.byte	8
	.byte	1
	.byte	3
	.word	47
	.byte	3
	.word	61
	.byte	4
	.word	47
	.byte	5
	.word	74
	.byte	5
	.byte	1
	.byte	46
	.byte	2
	.byte	4
	.byte	7
	.byte	8
	.byte	3
	.word	83
	.byte	6
	.byte	5
	.byte	8
	.byte	16
	.byte	3
	.byte	131
	.byte	16
	.byte	7
	.byte	6
	.word	100
	.byte	3
	.byte	132
	.byte	0
	.byte	0
	.byte	5
	.word	74
	.byte	7
	.byte	2
	.byte	98
	.byte	3
	.word	113
	.byte	5
	.word	47
	.byte	9
	.byte	2
	.byte	226
	.byte	3
	.word	126
	.byte	4
	.word	113
	.byte	8
	.byte	0
	.word	.Lfunc_end0-.Lfunc_begin0
	.byte	1
	.byte	82

	.byte	12
	.byte	3
	.byte	8
	.word	539

	.byte	9
	.byte	1
	.byte	90
	.byte	20
	.byte	3
	.byte	8
	.word	539
	.byte	9
	.byte	1
	.byte	91
	.byte	21
	.byte	3
	.byte	8
	.word	540
	.byte	10
	.byte	0
	.byte	22
	.byte	3
	.byte	8
	.word	66
	.byte	11
	.byte	13
	.byte	123
	.byte	0
	.byte	168
	.asciz	"\247\200\200"
	.byte	168
	.asciz	"\253\200\200"
	.byte	159
	.byte	23
	.byte	3
	.byte	9
	.word	47
	.byte	11
	.byte	1
	.byte	90
	.byte	24
	.byte	3
	.byte	10
	.word	51
	.byte	12
	.byte	0
	.word	.Ltmp4-.Lfunc_begin0
	.byte	13
	.byte	1
	.byte	25
	.byte	3
	.byte	11
	.word	540
	.byte	0
	.byte	0
	.byte	8
	.byte	1
	.word	.Lfunc_end1-.Lfunc_begin1
	.byte	1
	.byte	82

	.byte	13
	.byte	3
	.byte	19
	.word	539

	.byte	9
	.byte	1
	.byte	90
	.byte	20
	.byte	3
	.byte	19
	.word	539
	.byte	10
	.byte	2
	.byte	26
	.byte	3
	.byte	19
	.word	544
	.byte	9
	.byte	1
	.byte	92
	.byte	22
	.byte	3
	.byte	19
	.word	66
	.byte	13
	.byte	3
	.byte	24
	.byte	3
	.byte	20
	.word	51
	.byte	11
	.byte	1
	.byte	91
	.byte	27
	.byte	3
	.byte	21
	.word	56
	.byte	0
	.byte	14
	.byte	2
	.word	.Lfunc_end2-.Lfunc_begin2
	.byte	1
	.byte	82

	.byte	14
	.byte	15
	.byte	3
	.byte	153

	.byte	9
	.byte	1
	.byte	90
	.byte	28
	.byte	3
	.byte	153
	.word	108
	.byte	9
	.byte	1
	.byte	91
	.byte	26
	.byte	3
	.byte	153
	.word	121
	.byte	9
	.byte	1
	.byte	92
	.byte	29
	.byte	3
	.byte	153
	.word	66
	.byte	12
	.byte	3
	.word	.Ltmp18-.Ltmp13
	.byte	13
	.byte	4
	.byte	30
	.byte	3
	.byte	173
	.word	66
	.byte	13
	.byte	5
	.byte	31
	.byte	3
	.byte	175
	.word	550
	.byte	0
	.byte	0
	.byte	15
	.byte	4
	.word	.Lfunc_end3-.Lfunc_begin3
	.byte	1
	.byte	82

	.byte	16
	.byte	17
	.byte	3
	.byte	243
	.word	540

	.byte	11
	.byte	4
	.byte	145
	.asciz	"\220\300"
	.byte	35
	.byte	3
	.byte	247
	.word	590
	.byte	16
	.byte	3
	.byte	145
	.ascii	"\220 "
	.byte	36
	.byte	3
	.half	275
	.word	602
	.byte	16
	.byte	2
	.byte	145
	.byte	16
	.byte	40
	.byte	3
	.half	284
	.word	602
	.byte	17
	.byte	7
	.byte	41
	.byte	3
	.half	287
	.word	641
	.byte	17
	.byte	8
	.byte	42
	.byte	3
	.half	286
	.word	641
	.byte	18
	.byte	43
	.byte	3
	.half	256
	.word	646
	.byte	12
	.byte	5
	.word	.Ltmp28-.Ltmp22
	.byte	17
	.byte	6
	.byte	25
	.byte	3
	.half	276
	.word	66
	.byte	12
	.byte	6
	.word	.Ltmp25-.Ltmp23
	.byte	18
	.byte	44
	.byte	3
	.half	277
	.word	540
	.byte	0
	.byte	0
	.byte	12
	.byte	7
	.word	.Ltmp38-.Ltmp31
	.byte	17
	.byte	9
	.byte	25
	.byte	3
	.half	293
	.word	66
	.byte	0
	.byte	19
	.word	288
	.byte	8
	.byte	0
	.byte	20
	.byte	9
	.word	.Lfunc_end4-.Lfunc_begin4
	.byte	1
	.byte	82

	.byte	19
	.byte	3
	.half	376
	.word	540

	.byte	17
	.byte	10
	.byte	45
	.byte	3
	.half	378
	.word	540
	.byte	21
	.byte	1
	.byte	46
	.byte	3
	.half	379
	.word	540
	.byte	19
	.word	356
	.byte	10
	.byte	0
	.byte	22
	.byte	2
	.byte	18
	.byte	5
	.byte	4
	.byte	3
	.word	549
	.byte	23
	.byte	5
	.word	558
	.byte	34
	.byte	4
	.byte	102
	.byte	24
	.word	564
	.byte	33
	.byte	25

	.word	47
	.byte	26
	.word	586
	.byte	10
	.byte	146
	.ascii	"\2428"
	.byte	0
	.byte	49
	.byte	27
	.byte	56
	.byte	30
	.byte	49
	.byte	28
	.byte	0
	.byte	27
	.byte	32
	.byte	8
	.byte	7
	.byte	28
	.word	83
	.byte	29
	.word	586
	.byte	4
	.byte	0
	.byte	28
	.word	614
	.byte	29
	.word	586
	.byte	128
	.byte	0
	.byte	6
	.byte	5
	.byte	39
	.byte	32
	.byte	3
	.byte	136
	.byte	16
	.byte	7
	.byte	37
	.word	100
	.byte	3
	.byte	138
	.byte	0
	.byte	30
	.byte	38
	.word	78
	.byte	3
	.byte	140
	.byte	16
	.byte	16
	.byte	0
	.byte	3
	.word	614
	.byte	28
	.word	540
	.byte	29
	.word	586
	.byte	128
	.byte	0
	.byte	0
.Ldebug_info_end0:
	.section	.debug_str_offsets,"",@progbits
	.word	192
	.half	5
	.half	0
.Lstr_offsets_base0:
	.section	.debug_str,"MS",@progbits,1
.Linfo_string0:
	.asciz	"clang version 15.0.0 (https://github.com/llvm/llvm-project.git 853e0aa424e40b80d0bda1dd8a3471a361048e4b)"
.Linfo_string1:
	.asciz	"/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/vector_memcpy_pointers/vector_memcpy_pointers.cpp"
.Linfo_string2:
	.asciz	"/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/build/llvm-trunk-rv64imv/vector_memcpy_pointers"
.Linfo_string3:
	.asciz	"unsigned char"
.Linfo_string4:
	.asciz	"unsigned long"
.Linfo_string5:
	.asciz	"size_t"
.Linfo_string6:
	.asciz	"value"
.Linfo_string7:
	.asciz	"uint64_t"
.Linfo_string8:
	.asciz	"Base"
.Linfo_string9:
	.asciz	"uint8_t"
.Linfo_string10:
	.asciz	"DW_ATE_unsigned_8"
.Linfo_string11:
	.asciz	"DW_ATE_unsigned_32"
.Linfo_string12:
	.asciz	"memset"
.Linfo_string13:
	.asciz	"memcpy"
.Linfo_string14:
	.asciz	"_Z13vector_memcpyPhPKhm"
.Linfo_string15:
	.asciz	"vector_memcpy"
.Linfo_string16:
	.asciz	"_Z13run_base_testv"
.Linfo_string17:
	.asciz	"run_base_test"
.Linfo_string18:
	.asciz	"int"
.Linfo_string19:
	.asciz	"main"
.Linfo_string20:
	.asciz	"dest"
.Linfo_string21:
	.asciz	"ch"
.Linfo_string22:
	.asciz	"count"
.Linfo_string23:
	.asciz	"ch_uc"
.Linfo_string24:
	.asciz	"dest_uc"
.Linfo_string25:
	.asciz	"i"
.Linfo_string26:
	.asciz	"src"
.Linfo_string27:
	.asciz	"src_uc"
.Linfo_string28:
	.asciz	"dst"
.Linfo_string29:
	.asciz	"num_bytes"
.Linfo_string30:
	.asciz	"copied_per_iter"
.Linfo_string31:
	.asciz	"data"
.Linfo_string32:
	.asciz	"__ARRAY_SIZE_TYPE__"
.Linfo_string33:
	.asciz	"__rvv_uint8m8_t"
.Linfo_string34:
	.asciz	"vuint8m8_t"
.Linfo_string35:
	.asciz	"bases"
.Linfo_string36:
	.asciz	"source_array"
.Linfo_string37:
	.asciz	"expected_base_value"
.Linfo_string38:
	.asciz	"base_ptr"
.Linfo_string39:
	.asciz	"Element"
.Linfo_string40:
	.asciz	"dest_array"
.Linfo_string41:
	.asciz	"dst_ptr"
.Linfo_string42:
	.asciz	"src_ptr"
.Linfo_string43:
	.asciz	"indices"
.Linfo_string44:
	.asciz	"index"
.Linfo_string45:
	.asciz	"result"
.Linfo_string46:
	.asciz	"attempted"
	.section	.debug_str_offsets,"",@progbits
	.word	.Linfo_string0
	.word	.Linfo_string1
	.word	.Linfo_string2
	.word	.Linfo_string3
	.word	.Linfo_string4
	.word	.Linfo_string5
	.word	.Linfo_string6
	.word	.Linfo_string7
	.word	.Linfo_string8
	.word	.Linfo_string9
	.word	.Linfo_string10
	.word	.Linfo_string11
	.word	.Linfo_string12
	.word	.Linfo_string13
	.word	.Linfo_string14
	.word	.Linfo_string15
	.word	.Linfo_string16
	.word	.Linfo_string17
	.word	.Linfo_string18
	.word	.Linfo_string19
	.word	.Linfo_string20
	.word	.Linfo_string21
	.word	.Linfo_string22
	.word	.Linfo_string23
	.word	.Linfo_string24
	.word	.Linfo_string25
	.word	.Linfo_string26
	.word	.Linfo_string27
	.word	.Linfo_string28
	.word	.Linfo_string29
	.word	.Linfo_string30
	.word	.Linfo_string31
	.word	.Linfo_string32
	.word	.Linfo_string33
	.word	.Linfo_string34
	.word	.Linfo_string35
	.word	.Linfo_string36
	.word	.Linfo_string37
	.word	.Linfo_string38
	.word	.Linfo_string39
	.word	.Linfo_string40
	.word	.Linfo_string41
	.word	.Linfo_string42
	.word	.Linfo_string43
	.word	.Linfo_string44
	.word	.Linfo_string45
	.word	.Linfo_string46
	.section	.debug_addr,"",@progbits
	.word	.Ldebug_addr_end0-.Ldebug_addr_start0
.Ldebug_addr_start0:
	.half	5
	.byte	8
	.byte	0
.Laddr_table_base0:
	.quad	.Lfunc_begin0
	.quad	.Lfunc_begin1
	.quad	.Lfunc_begin2
	.quad	.Ltmp13
	.quad	.Lfunc_begin3
	.quad	.Ltmp22
	.quad	.Ltmp23
	.quad	.Ltmp31
	.quad	.Ltmp30
	.quad	.Lfunc_begin4
	.quad	.Ltmp44
.Ldebug_addr_end0:
	.ident	"clang version 15.0.0 (https://github.com/llvm/llvm-project.git 853e0aa424e40b80d0bda1dd8a3471a361048e4b)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym outputAttempted
	.addrsig_sym outputSucceeded
	.section	.debug_line,"",@progbits
.Lline_table_start0:
