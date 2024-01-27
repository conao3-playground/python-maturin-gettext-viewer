import maturin_gettext_viewer

file = maturin_gettext_viewer.po_file_read("sample/magit-section.ja.po")
iterator = maturin_gettext_viewer.po_message_iterator(file)
message = maturin_gettext_viewer.po_next_message(iterator)

print("msgid:")
print(maturin_gettext_viewer.po_message_msgid(message))

print("msgstr:")
print(maturin_gettext_viewer.po_message_msgstr(message))
