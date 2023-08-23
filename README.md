# OpenJarFix

OpenJarFix is a reimplementation of some of the functionality of [JarFix](https://johann.loefflmann.net/en/software/jarfix/index.html), developed by Johann Löfflmann.

OpenJarFix registers the .jar file extension with your installed JRE or JDK. It searches for javaw.exe in your PATH environment variable and adds the necessary registry keys for the file association.
This can be useful if
- You've installed Java with a package manager like [Scoop](https://scoop.sh/).
- Your .jar file extension has been hijacked by another application (for whatever reason).


## License
GPLv3

&copy; 2023 Florentin Schäfer
