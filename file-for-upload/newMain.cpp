#include "Color.h"
#include "Model.h"
#include "Raster.h"
#include "Triangle.h"
#include "Matrix.h"
#include <iostream>

using namespace std;

int main() {
  Raster myRaster(100,100,White);
  Model teapot1 = Model();
  Model teapotNew = Model();
  // Model teapot2 = Model();
  Model bunny = Model();
  Model cow = Model();
  teapot1.readFromOBJFile("./teapot.obj", Red);
  // teapot2.readFromOBJFile("./teapot.obj", Blue);
  bunny.readFromOBJFile("./bunny.obj", Red);
  cow.readFromOBJFile("./cow.obj", Red);
  teapotNew.readFromOBJFile("./teapot-new.obj", Blue);
  Vector4 eye(50, 50, 30, 1);
  Vector4 spot(50, 50, -30, 1);

  cout << teapot1.numTriangles() << endl;
  cout << teapotNew.numTriangles() << endl;
  cout << bunny.numTriangles() << endl;
  cout << cow.numTriangles() << endl;
  
  teapot1.performBackfaceCulling(eye, spot);
  teapotNew.performBackfaceCulling(eye, spot);
  bunny.performBackfaceCulling(eye, spot);
  cow.performBackfaceCulling(eye, spot);

  Matrix modelMatrixTeapot = Translate3D(50, 50, -60) * RotateZ3D(45.0) * Scale3D(0.5, 0.5, 0.5);
  Matrix modelMatrixTeapotNew = Translate3D(40, 40, -30) * RotateZ3D(45.0) * Scale3D(10, 10, 10);
  Matrix modelMatrixBunny = Translate3D(70, 30, -60) * RotateZ3D(-20.0) * Scale3D(500, 500, 500);
  Matrix modelMatrixCow = Translate3D(50, 50, -60) * RotateZ3D(45.0) * Scale3D(10, 10, 10);
  
  Matrix viewMatrix = LookAt(Vector4(50, 50, 30, 1), Vector4(50, 50, -40, 1), Vector4(0, 1, 0, 0));
  // Matrix viewMatrix = LookAt(Vector4(50, -20, -40, 1), Vector4(50, 50, -40, 1), Vector4(0, 0, 1, 0));
  // viewMatrix.print();
  // cout << endl;
  Matrix perspectiveMatrix = Perspective(70.0, myRaster.getWidth() / myRaster.getHeight(), 0.01, 88.5);
  // perspectiveMatrix.print();
  // cout << endl;
  Matrix viewPortMatrix = Viewport(0, 0, myRaster.getWidth(), myRaster.getHeight());
  // cout << "viewport matrix" << endl;
  // viewPortMatrix.print();
  // cout << endl;
  
  teapot1.transform(perspectiveMatrix * viewMatrix * modelMatrixTeapot);
  teapotNew.transform(perspectiveMatrix * viewMatrix * modelMatrixTeapotNew);
  bunny.transform(perspectiveMatrix * viewMatrix * modelMatrixBunny);
  cow.transform(perspectiveMatrix * viewMatrix * modelMatrixCow);

  teapot1.homogenize();
  teapotNew.homogenize();
  bunny.homogenize();
  cow.homogenize();

  teapot1.transform(viewPortMatrix);
  teapotNew.transform(viewPortMatrix);
  bunny.transform(viewPortMatrix);
  cow.transform(viewPortMatrix);

  // myRaster.drawModel(teapot1);
  myRaster.drawModel(teapotNew);
  myRaster.drawModel(bunny);
  // myRaster.drawModel(cow);

  cout << "model drawn" << endl;
  myRaster.writeToPPM();
}
