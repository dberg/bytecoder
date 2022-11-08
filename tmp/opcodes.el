;; https://github.com/akicho8/string-inflection
(require 'string-inflection)

;; The buffer input `opcodes` has the following format
;; // Constants
;; 00 (0x00)    nop
;; 01 (0x01)    aconst_null
;; 02 (0x02)    iconst_m1
;; 03 (0x03)    iconst_0
;; ...
;; The content was taken from the jvm spec
;; https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-7.html

;; ------------------------------------------------------------------------------
;; Helper. Should probably move to dbb package.
;; ------------------------------------------------------------------------------
(defun dbb-buffer-as-list (buffer-src)
  "Read a buffer and return its content as a string list of lines"
  (interactive "bBuffer to read content")
  (let* ((buf (with-current-buffer buffer-src (buffer-string)))
	 (lines (split-string buf "\n" t)))
    lines))

;; ------------------------------------------------------------------------------
;; Private
;; ------------------------------------------------------------------------------
(defun dbb-opcode-to-rust (buffer-src)
  "Returns list of (OPCODE, MNEMONIC, MNEMONIC-PASCAL-CASE)"
  (let* ((lines (dbb-buffer-as-list buffer-src)))
    (mapcar
     (lambda (line)
       (if (string-prefix-p "// " line)
	   line ;; keep comment line
	 (let* ((xs (split-string line))
		(opcode (nth 0 xs))
		(mnemonic (nth 2 xs))
		(mnemonic-pascal-case
		 (string-inflection-pascal-case-function (nth 2 xs))))
	   (list opcode mnemonic mnemonic-pascal-case))))
     lines)))

(defun dbb-opcode-to-rust-enum-value (triple) (nth 2 triple))
(defun dbb-opcode-to-rust-enum-value-function (triple)
  (string-join (list "Opcode::" (nth 2 triple) " => " )))
(defun dbb-opcode-to-rust-enum-string-function (triple)
  (string-join (list "Opcode::" (nth 2 triple) " => " (nth 1 triple))))
(defun dbb-opcode-to-rust-value-to-enum (triple)
  (string-join (list (nth 0 triple) " => Opcode::" (nth 2 triple))))

(defun dbb-opcode-to-rust-base (buffer-src triple-to-string-function)
  (let ((lines (dbb-opcode-to-rust buffer-src)))
    (mapcar
     (lambda (l)
       (if (stringp l)
	   (string-join (list l "\n"))
	 (string-join (list (funcall triple-to-string-function l) ",\n"))))
     lines)))

;; ------------------------------------------------------------------------------
;; public api to generate code
;; ------------------------------------------------------------------------------
;; (insert (mapconcat 'identity (dbb-opcode-to-rust-base "opcodes" 'dbb-opcode-to-rust-enum-value) ""))
;; (insert (mapconcat 'identity (dbb-opcode-to-rust-base "opcodes" 'dbb-opcode-to-rust-enum-value-function) ""))
;; (insert (mapconcat 'identity (dbb-opcode-to-rust-base "opcodes" 'dbb-opcode-to-rust-enum-string-function) ""))
;; (insert (mapconcat 'identity (dbb-opcode-to-rust-base "opcodes" 'dbb-opcode-to-rust-value-to-enum) ""))
