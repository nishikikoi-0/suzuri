-- INSERT INTO overrides (key, override_type, kanji, reading, value, replace) VALUES ('', '', '', '', '', <>);

-- FIX RESULTS FOR 犬
INSERT INTO overrides (key, override_type, kanji, reading, value, replace) VALUES ('dog', 'query', '犬', 'いぬ', '   Noun@     dog (Canis (lupus) familiaris), canine', 1);

-- DEFINITE ARTICLE EXPLANATIONS
INSERT INTO overrides (key, override_type, kanji, reading, value, replace) VALUES ('the', 'query', '', '', 'There is no direct equivalent of the word "the" in Japanese, as it lacks a definite article. Sentences which typically would use "a" or "the" in English would simply omit them in Japanese. To explicitly identify a referent, demonstratives (この,その,あの) are used.', 1);
INSERT INTO overrides (key, override_type, kanji, reading, value, replace) VALUES ('a', 'query', '', '', 'There is no direct equivalent of the word "a" in Japanese, as it lacks a definite article. Sentences which typically would use "a" or "the" in English would simply omit them in Japanese. To explicitly identify a referent, demonstratives (この,その,あの) are used.', 1);

-- FIX RESULTS FOR 何
INSERT INTO overrides (key, override_type, kanji, reading, value, replace) VALUES ('whatsit, whachamacallit, what''s-his-name, what''s-her-name', 'skip_value', '', '', '', 1);
INSERT INTO overrides (key, override_type, kanji, reading, value, replace) VALUES ('you-know-what, that thing', 'skip_value', '', '', '', 1);
