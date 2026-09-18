# Quora source ledger

Last reviewed: 2026-09-18.

## Official Quora sources

### Can I get a copy of my data?

https://help.quora.com/hc/en-us/articles/360000839503-Can-I-get-a-copy-of-my-data

Used for:

- existence of an archive of user content and personal data;
- request by privacy email/contact form;
- delivery to the account's primary email;
- typical 72-hour delivery after Quora confirms receipt.

### How do I see all the Quora questions I've asked or am following?

https://help.quora.com/hc/en-us/articles/360015510972-How-do-I-see-all-the-Quora-questions-I-ve-asked-or-am-following

Used for:

- `Your Content` includes questions, answers, and posts;
- exclusion of anonymously authored content;
- exclusion of privately followed questions.

### How does anonymity work on Quora?

https://help.quora.com/hc/en-us/articles/360000470226-How-does-anonymity-work-on-Quora

Updated April 13, 2026.

Used for:

- July 2021 end of anonymous question asking;
- November 2021 end of anonymous answering;
- statement that Quora systems do not retain identity information linking creators to anonymous questions/answers.

### Question and Answer Policies

https://help.quora.com/hc/en-us/articles/9456583756180-Question-and-Answer-Policies

Used for:

- questions are viewed as community property;
- current question/answer policy framework.

### How do I delete my own question from Quora?

https://help.quora.com/hc/en-us/articles/115004231186-How-do-I-delete-my-own-question-from-Quora

Used for:

- limited grace period for asker deletion;
- questions becoming community-governed after activity.

### What's the difference between deactivating and deleting my Quora account?

https://help.quora.com/hc/en-us/articles/360015476972-What-s-the-difference-between-deactivating-and-deleting-my-Quora-account

Used for:

- account deletion vs deactivation;
- community-property exception for questions/topics/Spaces.

### How do I delete my Quora account?

https://help.quora.com/hc/en-us/articles/115004250866-How-do-I-delete-my-Quora-account

Used for:

- 14-day grace period;
- account/profile/content deletion;
- legal-retention caveat;
- externally republished content.

### What are Bookmarks on Quora?

https://help.quora.com/hc/en-us/articles/360019495452-What-are-Bookmarks-on-Quora

Used for:

- existence of a private Bookmarks account surface.

### How do I follow people on Quora?

https://help.quora.com/hc/en-us/articles/115004229466-How-do-I-follow-people-on-Quora

Used for:

- people-follow relationship;
- feed use of followed-user activity.

### How do I follow/unfollow topics on Quora?

https://help.quora.com/hc/en-us/articles/115004211503-How-do-I-follow-unfollow-topics-on-Quora

Used for topic-follow state.

### How can I send a private message on Quora?

https://help.quora.com/hc/en-us/articles/115004227126-How-can-I-send-a-private-message-on-Quora

Used for:

- private Messages feature;
- user messaging privacy controls.

### Quora API Partners / Quora for Business

https://business.quora.com/api-partners

and current Quora for Business Conversion API material.

Used for:

- evidence that current prominently documented official APIs/integrations are focused on ads, reporting, attribution, conversion measurement, and brand-safety partners.

Important limitation: this does **not** prove that no other Quora API exists.

## Current-source limitation: canonical Terms

Quora's canonical Terms pages on `quora.com` were blocked to the research crawler by robots controls.

Historical/current-indexed copies indicate long-standing conditional-crawling and anti-scraping/automated-access rules, but those clauses are **not treated as fully current verified facts** until a human/manual recheck of the canonical Terms.

This limitation is recorded in `terms-and-constraints.md`.

## Public technical reverse-engineering source

### mparaz/qcrawlclaude — QUORA_API.md

https://github.com/mparaz/qcrawlclaude/blob/main/QUORA_API.md

Current public research documenting Quora's private web GraphQL protocol.

Used as **secondary technical evidence** for:

- `POST /graphql/gql_para_POST?q=<QueryName>`;
- `UserProfileQuestionsList_Questions_Query`;
- `UserProfileAnswersMostRecent_RecentAnswers_Query`;
- `AnswerComponentBaseQuery`;
- `UserProfileEditsQuery`;
- numeric `uid`, `qid`, `aid`, and `opid`;
- persisted-query hashes;
- cursor pagination;
- Qtext/rich-text payload structure;
- session/header names such as `Quora-Formkey`, revision/window/broadcast fields, and Turnstile token;
- observed disparity between UI and private-query history depth in specific tests;
- observed server-window behavior for activity history in one tested account.

Do not copy live hash values, session values, or extraction/bypass procedures from the third-party research into implementation.

## Evidence hierarchy

1. direct Sociarium observation of the operator's authorized first-party client;
2. current official Quora Help/Business material;
3. current official canonical Terms when manually verified;
4. multiple current public technical observations;
5. the current single detailed reverse-engineering source;
6. archived Terms/history;
7. historical memory.

Unknowns remain unknown until stronger evidence resolves them.
