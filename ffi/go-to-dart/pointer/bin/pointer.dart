import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'dart:io' show Directory;
import 'package:path/path.dart' as path;

typedef GetPointerFunc = Pointer<Utf8> Function();
final getStatus =
    dylib.lookupFunction<GetPointerFunc, GetPointerFunc>('GetPointer');

String libraryPath = "";
final dylib = DynamicLibrary.open(libraryPath);

void main(List<String> arguments) {
  libraryPath = path.join(Directory.current.path, './golang', 'lib.a');
  print(getStatus());
}
