
BUILDDIR := build


DEVICE := xc7a100t_test
BITSTREAM_DEVICE := artix7
PARTNAME := xc7a100tcsg324-1
OFL_BOARD := nexys_a7_100

SOURCE := top.v

RS_SOURCES := src/main.rs src/nexys_a7.rs src/blinky.rs src/counter.rs

TOP := top

XDC := ${TOP}.xdc
XDC_CMD := -x ${XDC}

# Build design
all: ${BUILDDIR}/${TOP}.bit

${BUILDDIR}:
	mkdir -p ${BUILDDIR}

${BUILDDIR}/${TOP}.v: ${RS_SOURCES} | ${BUILDDIR}
	cargo run build

${BUILDDIR}/${TOP}.eblif: ${BUILDDIR}/${TOP}.v ${BUILDDIR}/${XDC} ${SDC} ${PCF} | ${BUILDDIR}
	cd ${BUILDDIR} && symbiflow_synth -t ${TOP} ${SURELOG_OPT} -v ${TOP}.v -d ${BITSTREAM_DEVICE} -p ${PARTNAME} ${XDC_CMD}

${BUILDDIR}/${TOP}.net: ${BUILDDIR}/${TOP}.eblif
	cd ${BUILDDIR} && symbiflow_pack -e ${TOP}.eblif -d ${DEVICE} ${SDC_CMD} 2>&1 > /dev/null

${BUILDDIR}/${TOP}.place: ${BUILDDIR}/${TOP}.net
	cd ${BUILDDIR} && symbiflow_place -e ${TOP}.eblif -d ${DEVICE} ${PCF_CMD} -n ${TOP}.net -P ${PARTNAME} ${SDC_CMD} 2>&1 > /dev/null

${BUILDDIR}/${TOP}.route: ${BUILDDIR}/${TOP}.place
	cd ${BUILDDIR} && symbiflow_route -e ${TOP}.eblif -d ${DEVICE} ${SDC_CMD} 2>&1 > /dev/null

${BUILDDIR}/${TOP}.fasm: ${BUILDDIR}/${TOP}.route
	cd ${BUILDDIR} && symbiflow_write_fasm -e ${TOP}.eblif -d ${DEVICE}

${BUILDDIR}/${TOP}.bit: ${BUILDDIR}/${TOP}.fasm
	cd ${BUILDDIR} && symbiflow_write_bitstream -d ${BITSTREAM_DEVICE} -f ${TOP}.fasm -p ${PARTNAME} -b ${TOP}.bit

download: ${BUILDDIR}/${TOP}.bit
	openocd -f nexys7.cfg -c "init; pld load 0 ${BUILDDIR}/${TOP}.bit; exit"

flash: ${BUILDDIR}/${TOP}.bit
	openocd -f nexys7.cfg -c "init;\
    jtagspi_init 0 bscan_spi_xc7a100t.bit;\
    jtagspi_program ${BUILDDIR}/${TOP}.bit 0;\
    xc7_program xc7.tap;\
    shutdown"

clean-synth:
	rm -rf ${BUILDDIR}

clean: clean-synth
	rm -rf target