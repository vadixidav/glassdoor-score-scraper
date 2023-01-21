# glassdoor-score-scraper

Scrapes company scores off glassdoor

Inside some JSON in each company page, there is a section like the following:

```json
"employerReviews({\"applyDefaultCriteria\":true,\"division\":{\"id\":0},\"dynamicProfileId\":6088,\"employer\":{\"id\":6036},\"isRowProfileEnabled\":false,\"language\":\"eng\",\"page\":{\"num\":1,\"size\":1},\"preferredTldId\":0})": {
    "__typename": "EmployerReviews",
    "allReviewsCount": 156200,
    "ratings": {
        "__typename": "EmployerRatings",
        "ceoRating": 0.8,
        "ceoRatingsCount": 25539,
        "businessOutlookRating": 0.67,
        "compensationAndBenefitsRating": 3.9,
        "cultureAndValuesRating": 3.8,
        "careerOpportunitiesRating": 3.9,
        "overallRating": 3.8,
        "recommendToFriendRating": 0.74,
        "seniorManagementRating": 3.5,
        "workLifeBalanceRating": 3.4,
        "reviewCount": 138790,
        "diversityAndInclusionRating": 4.1,
        "ratedCeo": {
            "__ref": "Ceo:888147"
        }
    },
    "reviews": [
        {
            "__typename": "EmployerReview",
            "reviewId": 72838218,
            "employer": {
                "__typename": "Employer",
                "squareLogoUrl": "https:\u002F\u002Fmedia.glassdoor.com\u002Fsql\u002F6036\u002Famazon-squarelogo-1552847650117.png",
                "links": {
                    "__typename": "EiEmployerLinks",
                    "reviewsUrl": "\u002FReviews\u002FAmazon-Reviews-E6036.htm"
                }
            },
            "featured": false,
            "countHelpful": 0,
            "countNotHelpful": 0,
            "pros": "It is a very flexible job to work at.",
            "cons": "There's a lot of work load",
            "advice": null,
            "ratingBusinessOutlook": null,
            "ratingCeo": null,
            "ratingRecommendToFriend": null,
            "ratingOverall": 5,
            "ratingCultureAndValues": 0,
            "ratingDiversityAndInclusion": 0,
            "ratingCareerOpportunities": 0,
            "ratingCompensationAndBenefits": 0,
            "ratingSeniorLeadership": 0,
            "ratingWorkLifeBalance": 0,
            "summary": "Fun",
            "links": {
                "__typename": "EiReviewLinks",
                "reviewDetailUrl": "\u002FReviews\u002FEmployee-Review-Amazon-RVW72838218.htm"
            },
            "jobTitle": {
                "__typename": "JobTitle",
                "text": "Delivery Driver"
            },
            "location": null,
            "reviewDateTime": "2023-01-20T18:09:10.823",
            "lengthOfEmployment": 0,
            "isCurrentJob": true,
            "employmentStatus": "REGULAR"
        }
    ]
}
```

The goal is to seach up all of the companies in a list and extract the `overallRating` score for each. Additional scores would also be helpful. CSV file is the prefered output.
