#include <stdio.h>
#include <gettext-po.h>

static void
my_xerror (int severity,
           po_message_t message,
           const char *filename, size_t lineno, size_t column,
           int multiline_p, const char *message_text)
{
  (void)message;
  (void)filename;
  (void)lineno;
  (void)column;
  (void)multiline_p;
  printf ("xerror called:\n  %s\n", message_text);
  if (severity == PO_SEVERITY_FATAL_ERROR)
    abort ();
}


/* Signal a problem that refers to two messages.
   Similar to two calls to xerror.
   If possible, a "..." can be appended to MESSAGE_TEXT1 and prepended to
   MESSAGE_TEXT2.  */
static void
my_xerror2 (int severity,
            po_message_t message1,
            const char *filename1, size_t lineno1, size_t column1,
            int multiline_p1, const char *message_text1,
            po_message_t message2,
            const char *filename2, size_t lineno2, size_t column2,
            int multiline_p2, const char *message_text2)
{
  (void)filename1;
  (void)message1;
  (void)lineno1;
  (void)column1;
  (void)multiline_p1;
  (void)message2;
  (void)filename2;
  (void)lineno2;
  (void)column2;
  (void)multiline_p2;
  printf ("xerror2 called:\n  %s\n  %s\n", message_text1, message_text2);
  if (severity == PO_SEVERITY_FATAL_ERROR)
    abort ();
}


static struct po_xerror_handler handler = {
    my_xerror,
    my_xerror2,
};


int c_double_input(int input) {
    return input * 2;
}

po_file_t c_po_file_read(char* filename) {
    po_file_t po = po_file_read(filename, &handler);
    return po;
}

po_message_iterator_t c_po_message_iterator(po_file_t po) {
    po_message_iterator_t it = po_message_iterator(po, NULL);
    return it;
}

po_message_t c_po_next_message(po_message_iterator_t it) {
    po_message_t msg = po_next_message(it);
    return msg;
}

const char* c_po_message_msgid(po_message_t msg) {
    const char* msgid = po_message_msgid(msg);
    return msgid;
}

const char* c_po_message_msgstr(po_message_t msg) {
    const char* msgstr = po_message_msgstr(msg);
    return msgstr;
}
