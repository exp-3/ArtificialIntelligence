#include "Canvas.hpp"

#include <iostream>
#include <cmath>
#include <algorithm>
#include <opencv/cv.h>
#include <opencv/highgui.h>

using namespace std;

void error_callback(int error, const char* description)
{
  (void) error;
  std::cerr << "Error: " << description << std::endl;
}

Canvas::Canvas() {
  ;
}

void Canvas::init() {
  circle_virtices.resize(circle_virtices_num);
  for(int i = 0; i < (int)circle_virtices_num; ++i){
      GLfloat angle = static_cast<GLfloat>((M_PI * 2.0 * i) / circle_virtices_num);
      circle_virtices[i].x = circle_radius * sin(angle);
      circle_virtices[i].y = circle_radius * cos(angle);
  }


  if(glfwInit() == GL_FALSE) {
    cerr << "Can't initialize GLFW" << endl;
    exit(1);
  }

  atexit(glfwTerminate);

  glfwSetErrorCallback(error_callback);

  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 0);
  glfwWindowHint(GLFW_SAMPLES, 4);

  window = glfwCreateWindow(width, height, "TMS", NULL, NULL);
  if(window == NULL) {
    cerr << "Can't create GLFW window." << endl;
    exit(1);
  }

  glfwMakeContextCurrent(window);
  glfwSwapInterval(1);

  glMatrixMode(GL_PROJECTION);
  glLoadIdentity();
  glOrtho(-1.1f, 1.1f, -1.1f, 1.1f, -1.0f, 1.0f);

  glMatrixMode(GL_MODELVIEW);
  glLoadIdentity();

  glClearColor(1.0f, 1.0f, 1.0f, 0.0f);
  glLineWidth(5);
  update();
}

void Canvas::draw_circle(Point &p, Color &c) {
  glEnable(GL_MULTISAMPLE);
  glPushMatrix();

  glColor4f(c.r, c.g, c.b, 1.0f);
  glTranslated(p.x, p.y, 0);

  glBegin(GL_POLYGON);

  for(int i = 0; i < circle_virtices_num; i++)
    glVertex2d(circle_virtices[i].x, circle_virtices[i].y);

  glEnd();

  // glColor4f(0.0f, 0.0f, 0.0f, 1.0f);
  // glBegin(GL_LINE_LOOP);
  // for(int i = 0; i < (int)circle_virtices.size(); i++)
  //   glVertex2d(circle_virtices[i].x, circle_virtices[i].y);
  // glEnd();

  glPopMatrix();
  glDisable(GL_MULTISAMPLE);
}

void Canvas::draw_line(Point &start, Point &end) {
  glEnable(GL_MULTISAMPLE);
  glPushMatrix();

  glColor4f(0, 0, 0, 1.0f);

  glBegin(GL_LINES);
  glVertex2d(start.x, start.y);
  glVertex2d(end.x, end.y);
  glEnd();
  glPopMatrix();
  glDisable(GL_MULTISAMPLE);
}

void Canvas::clear() {
  glClear(GL_COLOR_BUFFER_BIT);
}

void Canvas::flush() {
  glfwSwapBuffers(window);
}

bool Canvas::should_close() {
  return glfwWindowShouldClose(window) == GL_TRUE;
}

int Canvas::get_window_width() {
  return width;
}

int Canvas::get_window_height() {
  return height;
}

void Canvas::poll_events() {
  glfwPollEvents();
}

void Canvas::wait_events() {
  glfwWaitEvents();
}

void Canvas::update() {
  glfwGetFramebufferSize(window, &width, &height);
  int side = min(width, height);
  glViewport((width - side) / 2, (height - side) / 2, side, side);
};

void Canvas::save(string filename) {
  const unsigned int channnelNum = 3; // RGBなら3, RGBAなら4
  void* dataBuffer = NULL;
  dataBuffer = ( GLubyte* )malloc( width * height * channnelNum);

  // 読み取るOpneGLのバッファを指定 GL_FRONT:フロントバッファ　GL_BACK:バックバッファ
  glReadBuffer( GL_FRONT );

  // OpenGLで画面に描画されている内容をバッファに格納
  glReadPixels(
	0,                 //読み取る領域の左下隅のx座標
	0,                 //読み取る領域の左下隅のy座標 //0 or getCurrentWidth() - 1
	width,             //読み取る領域の幅
	height,            //読み取る領域の高さ
	GL_BGR, //it means GL_BGR,           //取得したい色情報の形式
	GL_UNSIGNED_BYTE,  //読み取ったデータを保存する配列の型
	dataBuffer      //ビットマップのピクセルデータ（実際にはバイト配列）へのポインタ
	);

  GLubyte* p = static_cast<GLubyte*>( dataBuffer );
  // std::string fname = "outputImage.jpg";
  IplImage* outImage = cvCreateImage(cvSize( width, height), IPL_DEPTH_8U, 3 );

  for (int j = 0; j < height; ++ j)
  {
    for (int i = 0; i < width; ++i)
    {
      outImage->imageData[ ( height - j - 1 ) * outImage->widthStep + i * 3 + 0 ] = *p;
      outImage->imageData[ ( height - j - 1 ) * outImage->widthStep + i * 3 + 1 ] = *( p + 1 );
      outImage->imageData[ ( height - j - 1 ) * outImage->widthStep + i * 3 + 2 ] = *( p + 2 );
      p += 3;
    }
  }

  cvSaveImage( filename.c_str(), outImage );
}
