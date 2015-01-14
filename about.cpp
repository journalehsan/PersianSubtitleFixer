#include "about.h"
#include "ui_about.h"
#include "form1.h"
#include <QFile>
#include <qtextstream.h>

about::about(QWidget *parent) :
    QDialog(parent),
    ui(new Ui::about)
{
    ui->setupUi(this);
    this->setAutoFillBackground(true);
    this->setWindowTitle("Persian Subtitle Fixer");
    this->setStyleSheet("color: black;"
                                    "background-color: white;"
                                    "selection-color: blue;"
                                    "selection-background-color: white;");

    QFile fileopen(":/text/LICENSE");
         if (!fileopen.open(QIODevice::ReadOnly | QIODevice::Text)){
             //;
         }
    QTextStream open(&fileopen);
    QString filedata = open.readAll();
    fileopen.flush();
    fileopen.close();
    ui->plainTextEdit->setPlainText(filedata);
    ui->image->setScaledContents(true);
    ui->image->setMaximumHeight(70);
    ui->image->setMaximumWidth(70);
}

about::~about()
{
    delete ui;
}

void about::on_about_accepted()
{

}
