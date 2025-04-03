# Golem 1.2 internal testing hackathon results

- Critical missing features
    - Done
- Critical problems with existing features
    - Done
- Critical bugs
    - Done
    - To-do
- Critical documentation improvements
    - Done
    - TODO
- Non-critical missing features
    - Done
    - TODO
- Non-critical but annoying UX
    - Done
    - To-do / In-progress
- Wrong UX not immediately fixable

This document summarizes the found issues of the pre-1.2 internal testing
hackathon from each participants. The issues are categorized by severity to help
Golem 1.2 internal testing hackathon results 1scheduling the final set of tasks. (To be reviewed and adjusted together)
Each issue indicates who discovered it for easier tracking / gathering further info.

## Critical missing features

The items in this section are new things to be implemented before 1.2

### Done

- @vigoo@ziverge.com The original issue I ran into is that because the file-server bindings are pointing to a specific
  component version, this makes the iterative development very painful. In my example I was using it for hosting a
  frontend through an ephemeral component’s read-only IFS entries. Because of the specific component version, a new API
  definition must be created and deployed after every change of the frontend.
    - In general, I think the missing feature we should add is that for file-system ephemeral bindings we don’t require
      a component version but always use the latest one. (Maybe this is true for other binding types in case of
      ephemeral components - but for Rib I guess we still need a baseline for type
      checking) - https://github.com/golemcloud/golem/pull/1434
- @vigoo@ziverge.com It is not possible to define a binding for `/` . I wanted to bind that to my frontend (`index.html`
  in an ephemeral component) - https://github.com/golemcloud/golem/pull/1435
- @vigoo@ziverge.com - It is extremely hard to construct worker URI for RPC for ephemeral workers using the new
  component name resolution APIs. We Golem 1.2 internal testing hackathon results 2have everything needed to do for
  durable workers (component-id + name), but for ephemeral ones, we can resolve the component id by name, but then we
  cannot construct the urn:worker:<component-id> string. The reason is the WIT component-id type is a u64 pair (
  representing an uuid) and there is no way to easily convert that to a string (without knowing that it is a uuid and
  implementing an uuid to string conversion). In Rust the golem-rust crate at least has a conversion from the WIT Uuid
  to the “real” uuid::Uuid.
    - For now, we should add a variant of the worker_id host function that generates an ephemeral one by only taking a
      component-id.
    - Post 1.2 we should not use URIs in the location parameter of RPC resources
    - https://github.com/golemcloud/golem/pull/1432
- @vigoo@ziverge.com Missing --version or similar command in the new
  CLI - https://github.com/golemcloud/golem-cli/pull/153

## Critical problems with existing features

The items in this section are problems (not bugs, by design) with existing features
to be improved before 1.2

### Done

- @vigoo@ziverge.com The strict worker name rules are too restricting - for
  example it is not possible to use email address as worker name. This,
  combined with the fact that there are no string manipulation functions and
  things like base64 encoding in Rib yet, has a lot of consequences like the
  need for over-complicated architecture with proxy workers.
    - We should have a much lighter worker name rule (for example only prohibiting / and whitespace?)
    - https://github.com/golemcloud/golem/pull/1431
- @vigoo@ziverge.com Not enough information presented when the API definition yaml could not be parsed.
    - It should have enough information to immediately understand what to fix in the source file
    - Note: this may be already fixed, but let’s recheck.
    - https://github.com/golemcloud/golem-cli/pull/147
- @vigoo@ziverge.com If the request to API gateway does specified an Accept header as */* (default in curl ), I think it
  should assume that it is application/json. Right now this is not what is happening, and it returns with an error that
  it could not figure out how to response.
    - https://github.com/golemcloud/golem/pull/1431
- @vigoo@ziverge.com When registering an API definition and there is an error in one of the Rib scripts, the error
  message does not say which endpoint the problem belongs to.
    - https://github.com/golemcloud/golem/pull/1431
- @vigoo@ziverge.com API definitions are still referencing components for each endpoint by component-id. This is a
  problem because otherwise we removed them from the whole user experience. Components are managed “automatically” by
  deploy commands of the CLI.
    - Even if we re-add showing component IDs in CLI output, it is inconvenient to fill in the API definitions
      especially when there are many different components in the app
    - With golem run server --clean available and being used as part of the developer workflow, it means that during
      development and testing the components will get assigned new IDs from time to time. Currently in that case the API
      definitions need to be rewritten by hand to contain the new UUIDs.
    - So I think we should make the API definition use component names as part of 1.2
    - https://github.com/golemcloud/golem/pull/1444
- Make sure numbers are allowed for path and query parameters, even if Rib script requirements are string (global
  variables are by default string) - this is more of a regression that happened due to lacking tests.
    - by Afsal https://github.com/golemcloud/golem/pull/1447
- Remove unnecessary query and path parameter lookups (and collecting them to HashMap) if they are unused in Rib.
    - by Afsal https://github.com/golemcloud/golem/pull/1447

## Critical bugs

The items in this section are critical bugs to be fixed before 1.2

### Done

- @vigoo@ziverge.com ~Weird Rib error about global variables not allowed~
    - by @Afsal Thaj https://github.com/golemcloud/golem/pull/1430
- @vigoo@ziverge.com A comma after the last case of a match in Rib results in a very scary error message and it is very
  hard to figure out what causes it.
    - by @Afsal Thaj https://github.com/golemcloud/golem/pull/1437 (merged through another PR)
    - Details
        - For this script, this is the error returned:

```
let email = "user@test.com";
let temp_worker = instance("__accounts_proxy");
let user = temp_worker.get-user-name(email);
let worker = instance(user);
let result = worker.create-store(request.body);
match res {
Golem 1.2 internal testing hackathon results 5ok(_) ⇒ { status: 200, body: empty },
err(already-exists) ⇒ { status: 400, body: "Store already exists" },
}
```

```
error: API Definition Service - Error: 400 Bad Request, Parse error at line:
Unexpected `}`
Expected none, letter, digit, `_`, `-`, (, [, {, for, reduce, match, let, if, whitesp
Unable to parse custom constructor name
Unable to parse alias name
Invalid syntax for flag type
Invalid syntax for record type
Unable to parse flag or record
namespace
package
interface
Invalid function call
Invalid syntax for pattern match
Invalid syntax for tuple type
Invalid literal
Unable to parse not
Invalid syntax for Option type
Invalid syntax for Result type
Invalid syntax for sequence type
```

- @vigoo@ziverge.com Rib scripts are returning strings without the "" when content type is JSON, which is NOT a valid
  JSON.
    - by @Afsal Thaj : https://github.com/golemcloud/golem/pull/1438
- @Maxim Schuwalow api-definitions: query and path namespace sharing in rib is very
  surprising https://github.com/golemcloud/golem/blob/main/golem-worker-service-base/src/gateway_execution/request.rs#L134
    - by @Afsal Thaj : https://github.com/golemcloud/golem/pull/1439
- @Afsal Thaj (TBC) Either make sure the value in request.body is of required type (this is to be confirmed as we can
  already see some validation, but need to confirm it) and if this validation kills performance, let the rib interpreter
  returns proper error message.
    - by @Afsal Thaj : https://github.com/golemcloud/golem/pull/1441
- @Maxim Schuwalow: Very hard error messages when there is a type mismatch requirement
    - by @Afsal Thaj : https://github.com/golemcloud/golem/pull/1441

```
HTTP/1.1 400 Bad Request
content-length: 463
date: Tue, 18 Mar 2025 13:23:41 GMT
Invalid input: Input request doesn't match the requirements for rib expression
to execute: Key 'query' not found in json_map. Requirements. Record(TypeRe
cord { fields: [NameTypePair { name: "path", typ: Record(TypeRecord { fields:
[NameTypePair { name: "user-id", typ: Str(TypeStr) }] }) }, NameTypePair { na
me: "query", typ: Record(TypeRecord { fields: [NameTypePair { name: "after",
typ: U64(TypeU64) }, NameTypePair { name: "limit", typ: U8(TypeU8) }] }) }]
})
```

- @vigoo@ziverge.com Cannot change the component interface if it was used in an API binding, even if I delete all the
  API deployments and definitions. Looks like the constraint is never removed. (In progress by @Afsal Thaj)
    - by @Afsal Thaj : https://github.com/golemcloud/golem/pull/1442
- @vigoo@ziverge.com The → () syntax in our WIT files for function without return value are not compatible with latest
  tooling (for example wit-bindgen 0.40).
    - We should update them before the release
    - https://github.com/golemcloud/golem/pull/1432
- @Maxim Schuwalow Worker executor sometimes panics during golem app update-workers and cannot recover:
    - https://github.com/golemcloud/golem/pull/1451

```
thread 'tokio-runtime-worker' panicked at /home/mschuwalow/.cargo/git/che
ckouts/golem-f136eb08d75a2850/f783c7b/golem-worker-executor-base/src/
worker/mod.rs:1513:58:
called `Result::unwrap()` on an `Err` value: SendError { .. }
stack backtrace:
0: rust_begin_unwind
1: core::panicking::panic_fmt
2: core::result::unwrap_failed
```

### To-do

- Updating workers after reverting one can leave something (we have some local cli cache now, right? I guess that is the
  culprit) in a broken state
    - @Maxim Schuwalow taking this

```
Updating existing workers using auto mode
Updating all workers (1) for component chatroom:inbox to version 2
Triggering update for worker chatroom:inbox/abc to version 2 using auto up
date mode
Failed to trigger update for worker, error:
Worker Service - Error: 500 Internal Server Error, Invalid request: The sam
e update is already in progress
```

```
❯ golem worker list
+----------------+-------------+-----------+--------+-----------------------------+
| Component name | Worker name | Component | Status | Created at| | | | version | | |
+----------------+-------------+-----------+--------+-----------------------------+
| chatroom:inbox | abc | 0 | Idle | 2025-03-18 13:27:44.302 UTC |
+----------------+-------------+-----------+--------+-----------------------------+
| chatroom:room | room | 2 | Idle | 2025-03-18 13:07:41.380 UTC |
+----------------+-------------+-----------+--------+-----------------------------+
```

## Critical documentation improvements

This section contains documentation improvements that would be critical to do
before 1.2

### Done

- @vigoo@ziverge.com It is not clear from the documentation how exactly to define API definitions in the current, app
  manifest based world. Where to put the YAMLs? How many to create? What is the workflow regarding all the other
  deployment commands? This is something that will change if we integrate API definitions in the app manifest but even
  until we do so, we should have documentation about it.
- @vigoo@ziverge.com There is no information at all about how to run the OSS GUI
    - Even if we cannot solve the signed apps for 1.2, we should have instructions for it
    - PR - https://github.com/golemcloud/docs/pull/125 we are using the docs for now since the signing process taking
      time
- @vigoo@ziverge.com The “HTTP API definition binding types” page is hard to find in the docs. It is in the end of the
  References section and not cross-linked well enough in other places talking about API definition.
- Document how to add shared WIT packages into apps
    - @Maxim Schuwalow wit-deps setting in the golem.yaml does not automatically include the wit/common.wit file even
      though we automatically generate it
    - @Maxim Schuwalow Workflow for adding common wit to components is very confusing currently. I would expect to edit
      the wit file and / or Cargo.toml instead of having to edit the golem.yaml. I think we should automatically
      generate comments in those files that point out those pitfalls.
    - @vigoo@ziverge.com working on this
    - @vigoo@ziverge.com working on this
    - @vigoo@ziverge.com The file-server binding type description on the “HTTP API definition binding types” page is
      very confusing - it shows request.path.file but it does not have the part where that file is defined. I could only
      figure out what to actually do by checking an old CLI test (now deleted from the repo, by the way).

### TODO

- @vigoo@ziverge.com We should have a page about how to use app manifest, IFS, file server binding etc together to host
  a frontend.
- @Maxim Schuwalow api-definitions: query parameter bindings are undocumented
- Using variants and enums from Rib and passing them to invocation
- ~@Maxim Schuwalow if I search for “golem.yaml” on the documentation website I get no hits. Users will not know that
  this is called an application manifest, so discoverability should be better here.~

## Non-critical missing features

The items in this section are missing features that are not mandatory to be solved
before 1.2

### Done

- @vigoo@ziverge.com The CLI should show the component type (ephemeral or durable)
    - https://github.com/golemcloud/golem-cli/pull/148

### TODO

- @vigoo@ziverge.com There should be an easily extensible set of “standard library” functions exposed to Rib scripts,
  with some initial elements like base64 encoding/decoding. They could be either implemented as external native
  functions (in Rust) or in Rib itself if it becomes expressive enough.
- @vigoo@ziverge.com We should have a version of our reqwest fork that implements the async reqwest API. This could make
  it a drop-in replacement (via cargo patching) for many Rust libraries (for example AI clients)
- @vigoo@ziverge.com I think we are missing something to make secured endpoints testable. Not sure what exactly. (And
  could not test it yet, just based on the current docs). There should be a way to call the protected endpoints without
  requiring to go to a 3rd party identity provider, for testing purposes. It could be something depending on whether the
  deployment is “dev” or not, or an option to also allow basic auth or something else.
- @vigoo@ziverge.com There should be a way to define Rib functions and reuse them in multiple endpoints. Without that
  there is a lot of redundancy in endpoint definitions.
- ~@Maxim Schuwalow api definitions: Optional query parameter support. Maybe it’s just hidden and documentation is
  missing~
- @Afsal Thaj Generate a full fledged API definition based on component-id, with default METHOD in endpoints as GET and
  calling each function. The UI can have a Generate tab and cli can have the same functionality. Users can then start
  “editing” rather than developing a yaml or fill out UI forms from the scratch. They can always delete routes that they
  don’t need. I hope a user won’t have a 1000 functions exported in the component. In that case we put an upper limit to
  the number of routes generated. This will also give users an idea of how to write Rib without going through
  documentation. (In Postgres by @Afsal Thaj partly)

## Non-critical but annoying UX

The items in this section are annoyances that are nice to have for 1.2, but not
mandatory

### Done

- @vigoo@ziverge.com The API definition parser is very case sensitive. For example, I was writing GET as a method, and
  that is a syntax error (it only accepts Get ).
    - It should be more forgiving, especially for things like method which, I believe, many people write in uppercase by
      default
    - https://github.com/golemcloud/golem/pull/1431
- @vigoo@ziverge.com Why do I have to write --host localhost:9006 when deploying an API to a locally running Golem? It
  is confusing (and no other value works)
    - by @Afsal Thaj: https://github.com/golemcloud/golem-cli/pull/150
- @Afsal Thaj Remove `—def-format` option for api-definition in CLI:
    - by @Afsal Thaj https://github.com/golemcloud/golem-cli/pull/150
- @vigoo@ziverge.com the Uri type returned by the worker_uri host function is not the same as the Uri type required by
  the RPC stubs (they are isomorphic just two separately generated types of the same thing) so there has to be a
  conversion.
    - Would be nice to avoid this
    - Otherwise after 1.2 we should get rid of accepting Uri in those constructors anyway (Question: or before 1.2?)
    - https://github.com/golemcloud/golem/pull/1432
- @vigoo@ziverge.com The ValueAndOptionalType does not have pub fields which makes it impossible to use it (through
  golem-client crate) for testing
    - https://github.com/golemcloud/golem/pull/1432
- @Maxim Schuwalow automatic binding generation setup does not reuse generated types from golem_rust. This means that
  you cannot mix golem_rust returned types with the ones your own code is using. We should configure the binding
  settings in the Cargo.toml so the types are reused

```
golem-rust types mismatch:
| ---------------------- ^^^^^^^^^^^ expected bindings::golem::rpc::types::Uri, found
golem_rust::bindings::golem::rpc::types::Uri
```

    - https://github.com/golemcloud/golem/pull/1432

- @Maxim Schuwalow It would be nice to get rid of having to type the component name when interacting with generated
  clients. Here we should be able to figure out the name of the component from the golem.yaml / dynamic linking data
  uploaded to the server and offer a constructor for the client that just takes the worker name:
    - https://github.com/golemcloud/golem/pull/1432

```
let client_component_id = resolve_component_id("chatroom:inbox").unwrap
();
for client_name in state.connected_clients.iter() {
let client_uri = worker_uri(&WorkerId {
component_id: client_component_id.clone(),
worker_name: client_name.clone()
});
let client_api = ChatroomInboxApi::new(&client_uri);
...
```

- @vigoo@ziverge.com The Rust templates should add a #[allow(static_mut_ref)] before mod bindings to get rid of some
  warnings
- @vigoo@ziverge.com The default component templates are adding the common-lib crate as a dependency. Because of this,
  if I rename that crate to something, and then keep adding new components, those are no longer build because there is
  no longer a common-lib crate in the workspace.
    - I think we should only have the common-lib dependency in a “commented-out” form as a hint how to add it
- @Maxim Schuwalow We should include a link to documentation in the golem.yaml. The schema for the various editors work,
  but an overview that is easy to access would also be helpful. Alternatively we could generate commented out fields for
  the various settings with comments, as many settings files often do.

### To-do / In-progress

- @vigoo@ziverge.com the last published JSON schema (1.1.1) seems to be outdated (for example no static-wasm-rpc
  dependency type in it)
- @vigoo@ziverge.com the rust component templates now have a commented-out binding customization section in the end of
  their Cargo.toml But the cargo file rewrite adds new sections and it does not keep the comments together with the
  bindings section so it gets confusing.
    - Either the binding section header should be commented out too, or the target section (that gets manipulated by the
      tool) has to be added to the template
- @vigoo@ziverge.com Cargo.toml related up-to-date check issue - seems like if I only edit the binding customization
  section in the file, then golem app build is not rebuilding the component
- @vigoo@ziverge.com The error types generated into golem-client are bad in many ways. One user facing problem is, if
  the client crate is used for integration testing user applications, the errors cannot be ? d because they do not
  implement the Error trait.
    - At least we should implement the error trait for more convenient use, regardless of doing bigger changes to it
      later
- @vigoo@ziverge.com When the API definition points to wrong component IDs or versions (for example because the server
  was restarted with --clean ), the error response of registering the API definition is very bad: it contains the list
  of wrong component IDs for each endpoint, without mentioning which endpoint, with many duplicates (one entry per
  endpoint). For example:

```
error: API
Definition Service - Error: 400 Bad Request, Unable to fetch component: e3453d02-6e40-4827-bc29-
d1edf0d132dd#5, e3453d02-6e40-4827-bc29-d1edf0d132dd#5, b93c7c43-6dd7-437f-b126-35c7ccb18de5#11,
b93c7c43-6dd7-437f-b126-35c7ccb18de5#11, b93c7c43-6dd7-437f-b126-35c7ccb18de5#11, b93c7c43-6dd7-
437f-b126-35c7ccb18de5#10, 185c45ec-5c2d-460c-9415-093046f14142#6, 185c45ec-5c2d-460c-9415-
093046f14142#6, 185c45ec-5c2d-460c-9415-093046f14142#6
```

- @Maxim Schuwalow It would be nice to allow adding dependencies directly using the golem cli. An additional flag during
  component creation that automatically adjusts the golem.yaml. Otherwise discoverability of dependency settings is a
  bit bad. The same applies for common wit dependencies.
- @Afsal Thaj Have a separate compilation phase that string concatenation doesn’t include non literals. This behaviour
  needs to be confirmed. I think its a good idea to do this, especially users use it form worker-ids atleast as of
  now. (In progress by @Afsal Thaj)
- @Maxim Schuwalow : Static validation of query and path parameters than at runtime for Rib. It will reduce the
  difficulty in debugging when user accidentally uses request.path.* in Rib, and passed a request.query parameter.

## Wrong UX not immediately fixable

This section contains items that are important usability issues, but we don’t think
they can be fixed before 1.2

- @vigoo@ziverge.com The development flow of iterating on API definitions is very annoying. Basically it requires to:
    - Update the API yaml file
    - Manually bump the version in it
    - Register the new version ( golem api definition new api.yaml )
    - Delete the old deployment ( golem api deployment delete localhost:9006 )
    - Deploy the new version ( golem api deployment deploy my-api/0.0.1 )
    - Because of the version bump requirement and that the version is in the last command, it is not even easy to
      automate.
        - I think the solution is to integrate the API definitions in the app manifest and make it part of golem app
          deploy
- @Maxim Schuwalow trivial changes like adding a println! can fail worker updates. It would be nice to be more
  permissive here

```
2025-03-18T13:46:05.573910Z ERROR api_request{api="invoke_and_awai
t_worker_typed" api_type="grpc" worker_id="e34fb6b8-4e6d-4cea-915e
-ceb93e7e1d5e/room" account_id="-1"}:waiting-for-permits{worker_id="e
34fb6b8-4e6d-4cea-915e-ceb93e7e1d5e/room"}:invocation-loop{worker
_id="e34fb6b8-4e6d-4cea-915e-ceb93e7e1d5e/room"}:invocation{worke
Golem 1.2 internal testing hackathon results 15r_id=e34fb6b8-4e6d-4cea-915e-ceb93e7e1d5e/room}:replaying{function
="chatroom:room-exports/chatroom-room-api.{send}"}: golem_worker_ex
ecutor_base::durable_host::durability: Unexpected imported function call
entry in oplog: expected cli::preopens::get_directories, got golem_environ
ment::get_arguments
```

- https://github.com/golemcloud/golem/pull/1432

