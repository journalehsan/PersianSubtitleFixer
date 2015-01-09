#ifndef FORM1_H
#define FORM1_H

#include <QDialog>

namespace Ui {
class Form1;
}

class Form1 : public QWidget
{
    Q_OBJECT


public:
    explicit Form1(QWidget *parent = 0);

    ~Form1();

protected:
    void dragEnterEvent(QDragEnterEvent *event);
    void dropEvent(QDropEvent *event);

private slots:
    void on_toolButton_clicked();

    void on_toolButton_2_clicked();


    void on_toolButton_3_clicked();

    void on_toolButton_4_clicked();

private:
    Ui::Form1 *ui;

};

#endif // FORM1_H
