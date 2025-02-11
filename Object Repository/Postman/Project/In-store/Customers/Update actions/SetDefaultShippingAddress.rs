<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetDefaultShippingAddress</name>
   <tag></tag>
   <elementGuidId>61b7b141-848d-4d5f-bb00-ca86fd61a17d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${customer-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setDefaultShippingAddress\&quot;,\n            \&quot;addressId\&quot; : \&quot;${addressId}\&quot;\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>9037f8de-b076-4301-8ea0-4e410c3ca7d0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>67aced7d-afe4-474c-ad20-3de07ba3623f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/in-store/key=${store-key}/customers/${customer-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>4df7eeda-0b53-4a6e-91bd-d01c8cef009c</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>773ee1a7-28c2-47ea-b4ed-8de624e8db13</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.store-key</defaultValue>
      <description></description>
      <id>5eccc25c-19dc-4932-88aa-9b2b8fc72430</id>
      <masked>false</masked>
      <name>store-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.customer-id</defaultValue>
      <description></description>
      <id>8f56a267-de55-4dbd-8a0a-05479b9ff689</id>
      <masked>false</masked>
      <name>customer-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.customer-version</defaultValue>
      <description></description>
      <id>30ddda0b-5d7a-4416-abeb-eddbad9f82be</id>
      <masked>false</masked>
      <name>customer-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.addressId</defaultValue>
      <description></description>
      <id>dc06376b-1a1e-4e28-93cf-dc2c02cebccc</id>
      <masked>false</masked>
      <name>addressId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
