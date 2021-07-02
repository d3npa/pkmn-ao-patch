#!/bin/sh

make() {
	for dep in 'cc' 'rgbgfx'; do
		if [ -z `which $dep` ]; then
			echo "Could not find $dep in path"
			exit 1
		fi
	done

	set -x

	cc -o tools/pkmncompress tools/pkmncompress.c

	for file in `find . -name '*.png'`; do
		rgbgfx -o "`dirname $file`/`basename $file .png`.2bpp" $file
	done

	for file in `find . -wholename '*/gfx/player/*.2bpp'`; do
		tools/pkmncompress $file "`dirname $file`/`basename $file .2bpp`.pic"
	done
}

clean() {
	set -x

	for file in `find . -name '*.2bpp' -or -name '*.pic'`; do
		rm $file
	done

	if [ -e tools/pkmncompress ]; then
		rm tools/pkmncompress
	fi
}

case $1 in
	clean)
		clean
		;;
	*)
		make
		;;
esac