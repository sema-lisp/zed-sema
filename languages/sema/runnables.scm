; Zed doesn't draw a runnable for the whole source file (the root node), so we
; anchor a ▶ to the head symbol of each top-level form. Clicking it runs the
; current file via the `sema-run` task (see the README for the task snippet).
(source_file
  (list . (symbol) @run)
  (#set! tag sema-run))
