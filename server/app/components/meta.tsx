import React, { FC } from "react";
import Head from "next/head";

export interface MetaTitleProps {
  readonly page?: string | string[] | null;
  readonly site?: string | null;
}

export const MetaTitle: FC<MetaTitleProps> = ({ page, site }) => {
  const pageTitle = Array.isArray(page) ? page.join(" | ") : page;
  const pageTitleOrSite = pageTitle || site;
  return (
    <Head>
      <title>
        {!!pageTitle && site ? `${pageTitle} | ${site}` : pageTitle || site}
      </title>
      {!!pageTitle && (
        <meta name="og:title" property="og:title" content={pageTitle} />
      )}
      {site && <meta property="og:site_name" content={site} />}
      {pageTitleOrSite && (
        <meta
          name="twitter:title"
          property="twitter:title"
          content={pageTitleOrSite}
        />
      )}
    </Head>
  );
};

export interface MetaDescriptionProps {
  readonly description: string;
}

export const MetaDescription: FC<MetaDescriptionProps> = ({ description }) => {
  if (!description) return null;
  return (
    <Head>
      <meta name="description" content={description} />
      <meta
        name="og:description"
        property="og:description"
        content={description}
      />
      <meta
        name="twitter:description"
        property="twitter:description"
        content={description}
      />
    </Head>
  );
};

export interface MetaTypeProps {
  readonly type: "article" | "website";
}

export const MetaType: FC<MetaTypeProps> = ({ type }) => (
  <Head>
    <meta property="og:type" content={type} />
  </Head>
);
