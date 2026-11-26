# Expected: one piece

## Request

Add a retry count to the settings of our worker, read it where the worker
retries, and keep today's value as the default.

## What a good brief does

Keeps it as one piece of work. No split is proposed on grounds of order.

## Why

It is a new field and its first use, like the case that is split, but nothing
that runs today has to understand the field before the change can be built,
checked and reviewed: the code that reads it is built and tested from the same
branch.
