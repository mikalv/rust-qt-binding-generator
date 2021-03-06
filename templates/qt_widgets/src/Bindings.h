/* generated by rust_qt_binding_generator */
#ifndef BINDINGS_H
#define BINDINGS_H

#include <QObject>
#include <QAbstractItemModel>

class Simple;

class Simple : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(QString message READ message WRITE setMessage NOTIFY messageChanged FINAL)
    explicit Simple(bool owned, QObject *parent);
public:
    explicit Simple(QObject *parent = nullptr);
    ~Simple();
    QString message() const;
    void setMessage(const QString& v);
signals:
    void messageChanged();
};
#endif // BINDINGS_H
