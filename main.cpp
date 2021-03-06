﻿//This program is free software: you can redistribute it and/or modify
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
#include <QApplication>
#include <QTextCodec>
#include <QFileInfo>
void setFileName(QString strFileName);
int main(int argc, char *argv[])
{
    QString srtFileName = argv[1];
    if(srtFileName!= ""){
        setFileName(srtFileName);
    }
    else
    {
        setFileName(argv[1]);
    }
    QApplication a(argc, argv);
    Form1 form;
    if (!argv[1]){
        form.setWindowTitle("Persian Subtitle Fixer");
    }
    else
    {
        QFileInfo fileInfo(argv[1]);
        QString fileName(fileInfo.fileName());
        form.setWindowTitle(fileName);
    }
    form.show();
    return a.exec();
}
