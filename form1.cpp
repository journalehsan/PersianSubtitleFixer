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

QString globalfilename;
QString globalname2;
QString openfile(QString filename, bool choiceiszero);

Form1::Form1(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::Form1)
{
    ui->setupUi(this);
    ui->cmbcodec->addItem(" زیر نویس خراب با حروف ناخوانا");
    ui->cmbcodec->addItem("زیر نویس سالم - UTF-8");
    ui->textEdit->setPlainText(" Drag File Here ");
    ui->textEdit->setAcceptDrops(false);
    setAcceptDrops(true);
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
   globalname2 = fileName;
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

void Form1::on_toolButton_2_clicked()
{
    QString homepath =  globalfilename;
    QMessageBox::critical(this,"Error",homepath);
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
      QMessageBox::critical(this,"Error", filename);
    }

    QTextStream out(&filesave);
   QString text = ui->textEdit->toPlainText();
   out.setCodec("utf-8");
    out << text;
    filesave.close();
    filesave.flush();
}

void Form1::on_toolButton_clicked()
{
    QString filename = globalname2;
    QString homepath =  QDir::homePath();
        filename= QFileDialog::getOpenFileName(this,"Open Windows-1256-Arabic SRT File:",homepath,"SRT File(*.srt);;AllFile(*.*)");
    globalfilename = filename;

    //Change Unicode based on combobox
    bool choiceiszero;
     if (ui->cmbcodec->currentIndex() == 0){
         choiceiszero = true;
     }
     else{
         choiceiszero = false;
     }
     QString FileData = openfile(filename,choiceiszero);

    //Display datat in Textbox
    ui->textEdit->setPlainText(FileData);
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

void Form1::on_toolButton_4_clicked()
{
//
}
