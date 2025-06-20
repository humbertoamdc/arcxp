ACCOUNT_ID := 846447858735
REGION := us-east-1

# === App ===

run:
	./scripts/run-dev.sh

test:
	./scripts/run-tests.sh

# === Infra ===

create-stacks:
	make lambda
	make create-stack stack=dynamodb env=$$env
	make create-stack stack=certificate-manager
	make create-stack stack=api-gateway env=$$env

# Parameters
# 	- stack: string = Name of stack to create.
create-stack:
	aws --region $(REGION) cloudformation deploy \
		--capabilities CAPABILITY_NAMED_IAM \
		--template-file infra/$(stack).yaml \
		--stack-name $(stack)

lambda:
	make create-stack stack=lambda; \
	cargo lambda build --release --arm64; \
	cargo lambda deploy --region $(REGION) \
		--iam-role arn:aws:iam::$(ACCOUNT_ID):role/ArcxpLambdaRole; \
	sleep 10; \
	aws --region $(REGION) lambda update-function-configuration \
		--function-name arcxp \
		--environment "Variables={DEPLOY_TARGET=lambda,ENV=production}" \
		--no-cli-pager
