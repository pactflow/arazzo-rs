To generate the log, run `git log --pretty='* %h - %s (%an, %ad)' TAGNAME..HEAD .` replacing TAGNAME and HEAD as appropriate.

# 0.0.6 - Replace Either with local enum

* 9105f52 - refactor: Switch out Intertools::Either for a local enum (Ronald Holshausen, Mon Aug 11 10:21:01 2025 +1000)
* 399022d - chore: Update docs (Ronald Holshausen, Fri Aug 8 11:39:52 2025 +1000)
* fb1a7fd - chore: Correct the doc examples (Ronald Holshausen, Fri Aug 8 11:30:34 2025 +1000)
* 2ba49e6 - bump version to 0.0.6 (Ronald Holshausen, Fri Aug 8 11:26:00 2025 +1000)

# 0.0.5 - Updated docs and added missing PayloadReplacement

* 37841a0 - feat: Added PayloadReplacement (Ronald Holshausen, Fri Aug 8 11:20:15 2025 +1000)
* cc309f3 - feat: Update readme  examples (Ronald Holshausen, Thu Aug 7 15:54:10 2025 +1000)
* 401d851 - feat: Add examples to the docs (Ronald Holshausen, Thu Aug 7 15:51:21 2025 +1000)
* f85e774 - bump version to 0.0.5 (Ronald Holshausen, Thu Aug 7 12:24:39 2025 +1000)

# 0.0.4 - Complete 1.0 spec implementation

* de26784 - feat: Added all fields for Components (Ronald Holshausen, Thu Aug 7 12:16:49 2025 +1000)
* d9dd8eb - feat: Added criteria to SuccessObject and FailureObject (Ronald Holshausen, Thu Aug 7 11:43:56 2025 +1000)
* 050a60b - bump version to 0.0.4 (Ronald Holshausen, Thu Aug 7 10:48:08 2025 +1000)

# 0.0.3 - Support loading from JSON

* fe393a0 - feat: Completed loading models from JSON (Ronald Holshausen, Thu Aug 7 10:43:36 2025 +1000)
* d130300 - feat: Add loading Info and SourceDescription from JSON (Ronald Holshausen, Thu Aug 7 09:56:25 2025 +1000)
* 9bd9f1e - feat: Add loading Workflow from JSON (Ronald Holshausen, Thu Aug 7 09:47:50 2025 +1000)
* 982fc15 - feat: Added loading Step from JSON (Ronald Holshausen, Wed Aug 6 17:38:14 2025 +1000)
* 4e3310b - feat: Added loading SuccessObject and FailureObject from JSON (Ronald Holshausen, Wed Aug 6 17:13:08 2025 +1000)
* 8f68338 - feat: Added loading ReusableObject and ParameterObject from JSON (Ronald Holshausen, Wed Aug 6 16:59:58 2025 +1000)
* 6daf205 - feat: Added loading Request Body and Payload from JSON (Ronald Holshausen, Wed Aug 6 16:44:41 2025 +1000)
* 2db3c4d - feat: Added loading Criterion from JSON (Ronald Holshausen, Wed Aug 6 16:25:58 2025 +1000)
* 755baef - refactor: Move all YAML functions to the yaml module (Ronald Holshausen, Wed Aug 6 15:48:13 2025 +1000)
* e549b99 - bump version to 0.0.3 (Ronald Holshausen, Wed Aug 6 15:26:28 2025 +1000)

# 0.0.2 - All spec objects completed

* a50365b - feat: Added Criterion (Ronald Holshausen, Wed Aug 6 15:20:43 2025 +1000)
* 8bd2bdb - feat: Added success, failure and outputs to Step (Ronald Holshausen, Wed Aug 6 14:29:46 2025 +1000)
* 4a319c5 - feat: Added some conversion functions to Payload (Ronald Holshausen, Wed Aug 6 14:10:49 2025 +1000)
* 1d9c94e - feat: Added RequestBody and Payloads (Ronald Holshausen, Wed Aug 6 14:04:28 2025 +1000)
* 86fc4d2 - feat: Added Step parameters (Ronald Holshausen, Wed Aug 6 10:55:46 2025 +1000)
* 3b36466 - feat: Added simple Step fields (Ronald Holshausen, Wed Aug 6 10:41:53 2025 +1000)
* b53f2bb - feat: Added Workflow parameters and outputs (Ronald Holshausen, Wed Aug 6 10:19:27 2025 +1000)
* e17f453 - bump version to 0.0.2 (Ronald Holshausen, Wed Aug 6 09:27:00 2025 +1000)

# 0.0.1 - Added Workflow and Step Objects

* 76a9d77 - chore: Prep for running release script (Ronald Holshausen, Wed Aug 6 09:24:02 2025 +1000)
* 1f4ee21 - chore: Fix imports (Ronald Holshausen, Wed Aug 6 09:20:00 2025 +1000)
* 1fc37b2 - feat: Add Success, Failure and Reusable Objects (Ronald Holshausen, Tue Aug 5 16:51:22 2025 +1000)
* f0d401e - chore: Fix imports (Ronald Holshausen, Tue Aug 5 15:47:20 2025 +1000)
* 3ffd33b - feat: Add Workflow inputs and basic Steps (Ronald Holshausen, Tue Aug 5 15:44:10 2025 +1000)
* e106d63 - feat: Add basic components object (Ronald Holshausen, Tue Aug 5 14:42:28 2025 +1000)
* d6d96bb - feat: Add basic workflow object (Ronald Holshausen, Tue Aug 5 12:06:33 2025 +1000)
* dbe922a - feat: Implement Source Description Object (Ronald Holshausen, Tue Aug 5 11:39:06 2025 +1000)
* e9fc955 - chore: Correct imports (Ronald Holshausen, Tue Aug 5 10:54:26 2025 +1000)
* 10e432e - feat: Support extension values (Ronald Holshausen, Tue Aug 5 10:50:38 2025 +1000)

# 0.0.0 - Initial Release to crates.rs
 
* 48fa638 - chore: Fix imports (Ronald Holshausen, Mon Aug 4 17:24:23 2025 +1000)
* 14ab51b - feat: Add Info model (Ronald Holshausen, Mon Aug 4 17:15:31 2025 +1000)
* 7a92a34 - chore: Support future spec versions (Ronald Holshausen, Mon Aug 4 16:34:59 2025 +1000)
* f3c36e2 - chore: Add basic loading main descriptor from YAML (Ronald Holshausen, Mon Aug 4 16:21:37 2025 +1000)
* a660182 - chore: Add basic readme and crate metadata (Ronald Holshausen, Mon Aug 4 15:32:10 2025 +1000)
* 4521075 - chore: As with every repo, the follow up commit to Add CI build ... Part 3 (Ronald Holshausen, Mon Aug 4 15:15:52 2025 +1000)
* 0431f1c - chore: As with every repo, the follow up commit to Add CI build ... Part 2 (Ronald Holshausen, Mon Aug 4 15:13:49 2025 +1000)
* e75f17a - chore: As with every repo, the follow up commit to Add CI build ... (Ronald Holshausen, Mon Aug 4 15:10:52 2025 +1000)
* 63a5e65 - chore: Add CI build (Ronald Holshausen, Mon Aug 4 15:07:17 2025 +1000)
* 3cf75e3 - chore: Create initial model crate (Ronald Holshausen, Mon Aug 4 15:00:50 2025 +1000)
* a50023d - Initial commit (Ronald Holshausen, Mon Aug 4 14:53:16 2025 +1000)
