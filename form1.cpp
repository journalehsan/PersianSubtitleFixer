//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//(at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.

//You should have received a copy of the GNU General Public License
//along with this program.  If not, see <http://www.gnu.org/licenses/>.

//Author:Ehsan Tork journalehsan@gmail.com
#include "form1.h"
#include "ui_form1.h"
#include <QtGui>
#include <QString>
#include <QFile>
#include <QFileDialog>
#include <QMessageBox>
#include <QDataStream>
#include <QTextCodec>
#include <QDir>
#include <QFontDialog>
#include <QShortcut>
#include "about.h"

QString opendfilename;
QString dragfilename;
QString firstfilename;
QString openfile(QString filename, bool choiceiszero);

Form1::Form1(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::Form1)
{
    ui->setupUi(this);
    QString fileData;
    ui->cmbcodec->addItem("Windows Arabic - نا سالم");
    new QShortcut(QKeySequence(Qt::CTRL + Qt::Key_Q), this, SLOT(close()));
    new QShortcut(QKeySequence(Qt::Key_Escape), this, SLOT(close()));
    new QShortcut(QKeySequence(Qt::CTRL + Qt::Key_O), this, SLOT(on_toolButton_clicked()));
    new QShortcut(QKeySequence(Qt::CTRL + Qt::Key_S), this, SLOT(on_toolButton_2_clicked()));
    new QShortcut(QKeySequence(Qt::CTRL + Qt::Key_F), this, SLOT(on_toolButton_3_clicked()));
    ui->cmbcodec->addItem("UTF-8 - سالم");
    this->setWindowTitle("Persian Subtitle Fixer");
    ui->textEdit->setAcceptDrops(false);

    setAcceptDrops(true);
    QString filename = firstfilename;
    //open file

    if(QFile::exists(filename)){
        fileData = openfile(filename,true);
    }
    else
    {
        //
    }
    if(fileData != ""){
        ui->textEdit->setPlainText(fileData);
        this->setWindowTitle(filename);
    }
    else
    {
        ui->textEdit->setPlainText("Drag SRT Subtitle Here");
    }
}
void setfilename(QString filename){
firstfilename = filename;
}

Form1::~Form1()
{
    delete ui;
}
void Form1::dragEnterEvent(QDragEnterEvent *event)
{
    if (event->mimeData()->hasFormat("text/uri-list"))
        event->acceptProposedAction();
}
void Form1::dropEvent(QDropEvent *event)
{
    QList<QUrl> urls = event->mimeData()->urls();
    if (urls.isEmpty())
        return;

    QString fileName = urls.first().toLocalFile();
    if (fileName.isEmpty())
        return;
   dragfilename = fileName;
   bool choiceiszero;
   if (ui->cmbcodec->currentIndex()==0) {
       choiceiszero = true;}
   else{
       choiceiszero = false;
   }
   QString plaindata = openfile(fileName,choiceiszero);
   ui->textEdit->setPlainText(plaindata);

}
QString openfile(QString filename,bool choiceiszero){
    QFile fileopen(filename);
         if (!fileopen.open(QIODevice::ReadOnly | QIODevice::Text)){
             return "";
         }
    QTextStream open(&fileopen);
    if (choiceiszero == true){
        open.setCodec("windows-1256");
    }
    else{
        open.setCodec("utf-8");
    }
    QString filedata = open.readAll();
    fileopen.flush();
    fileopen.close();
    return filedata;
}

void Form1::on_toolButton_clicked()
{

    QString filename = dragfilename;
    QString homepath =  QDir::homePath();
        filename= QFileDialog::getOpenFileName(this,"Open Windows-1256-Arabic SRT File:",homepath,"SRT File(*.srt);;AllFile(*.*)");
    opendfilename = filename;
    //Change Unicode based on combobox
    bool choiceiszero;
     if (ui->cmbcodec->currentIndex() == 0){
         choiceiszero = true;
     }
     else{
         choiceiszero = false;
     }
     if (filename !="" || !QFile::exists(filename) ){
         QString FileData = openfile(filename,choiceiszero);
         //Display datat in Textbox
         ui->textEdit->setPlainText(FileData);
         this->setWindowTitle(filename);
     }
}
void Form1::on_toolButton_2_clicked()
{
    QString homepath =  opendfilename;
    if(homepath == ""){
        QMessageBox::critical(this,"Error", "Please open file first! \n You can't save befor open a file!");
        on_toolButton_clicked();
    }
    else{
        QString filename =  QFileDialog::getSaveFileName(this, tr("Save File"),
                                                         homepath,
                                                         tr("Save SRT to UTF-8 (*.srt)"));
        QFile filedelete(filename);
        if (filedelete.exists())
            filedelete.remove();
        filedelete.close();
        //savedialog
        QFile filesave(filename);
        if (!filesave.open(QIODevice::ReadWrite | QIODevice::Text)){

        }
        else{
            QTextStream out(&filesave);
           QString text = ui->textEdit->toPlainText();
           out.setCodec("utf-8");
            out << text;
            filesave.close();
            filesave.flush();
        }
    }
}

void Form1::on_toolButton_3_clicked()
{
    bool ok;
    QFont font = QFontDialog::getFont(
                    &ok, QFont("Helvetica [Cronyx]", 10), this);
    if (ok)  {
        ui->textEdit->setFont(font);
    } else  {
        // the user canceled the dialog; font is set to the initial
        // value, in this case Helvetica [Cronyx], 10
    }
}

void Form1::on_pushButton_2_clicked()
{
    about *w = new about;
            w->setAttribute(Qt::WA_DeleteOnClose);
            w->show();
}
