# Expected: split

## Request

Move to the new release of the library that defines our manifest, and use the new
optional command it adds in our own manifest.

## What a good brief does

Splits it in two, with a blocked-by link. First: move to the release and accept
the field. Then: use it. It names what has to happen between them: the installed
build, which checks a branch's manifest against its own schema, is rebuilt.

## Why

The installed build rejects the new field until it is rebuilt, so a change that
holds both halves fails every run before anyone reviews it.
