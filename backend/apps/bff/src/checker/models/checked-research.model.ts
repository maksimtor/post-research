import { Field, ID, ObjectType } from '@nestjs/graphql';

@ObjectType({ description: 'Check ' })
export class CheckedAuthor {
  @Field(type => String)
  name: string;

  @Field(type => String)
  id: string;

  @Field(type => [RetractedArticle])
  retractedArticles: RetractedArticle[];
}

@ObjectType({ description: 'Check ' })
export class CheckedAffiliation {
  @Field(type => String)
  name: string;

  @Field(type => [RetractedArticle])
  retractedArticles: RetractedArticle[];
}

@ObjectType({ description: 'Check ' })
export class CheckedCitation {
  @Field(type => String)
  name: string;

  @Field(type => String)
  doi: string;

  @Field(type => String, { nullable: true })
  pubmedId?: string;

  @Field(type => Boolean)
  retracted: boolean;
}

@ObjectType({ description: 'Check ' })
export class CheckedFunding {
  @Field(type => String)
  name: string;

  @Field(type => [RetractedArticle])
  retractedArticles: RetractedArticle[];
}

@ObjectType({ description: 'Check ' })
export class CheckedPubBase {
  @Field(type => String)
  name: string;

  @Field(type => [RetractedArticle])
  retractedArticles: RetractedArticle[];
}

@ObjectType({ description: 'Check ' })
export class RetractedArticle {
  @Field(type => String)
  name: string;

  @Field(type => String)
  doi: string;

  @Field(type => String, { nullable: true })
  pubmedId?: string;
}

@ObjectType({ description: 'Check ' })
export class CheckedResearch {
  @Field(type => String)
  name: string;

  @Field(type => [CheckedAuthor])
  authors: CheckedAuthor[];

  @Field(type => [CheckedAffiliation])
  affiliations: CheckedAffiliation[];

  @Field(type => [CheckedCitation])
  citations: CheckedCitation[];

  @Field(type => [CheckedFunding])
  fundings: CheckedFunding[];

  @Field(type => CheckedPubBase)
  pubBase: CheckedPubBase;

  @Field(type => String)
  doi: string;

  @Field(type => String, { nullable: true })
  pubmedId?: string;
}
